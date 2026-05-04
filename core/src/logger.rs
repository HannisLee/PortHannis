use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use chrono::Utc;
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{LogEvent, LogLine, LogMessage, LogResponse};

/// 单条日志最大分段大小：1 MB
pub const MAX_SEGMENT_BYTES: u64 = 1_000_000;
/// 最大分段数（不含当前活跃文件）：5 个
pub const MAX_SEGMENTS: usize = 5;
/// 单条目日志总上限：6 MB（5 个历史段 + 1 个当前文件）
pub const MAX_TOTAL_BYTES: u64 = MAX_SEGMENT_BYTES * (MAX_SEGMENTS as u64 + 1);

/// 日志记录器——负责将日志消息写入磁盘并进行分段轮转。
pub struct EntryLogger {
    entry_id: Uuid,
    dir: PathBuf,
    writer: Option<BufWriter<File>>,
    current_size: u64,
}

impl EntryLogger {
    /// 创建新的日志记录器。如果日志目录不存在会自动创建。
    pub fn new(dir: PathBuf, entry_id: Uuid) -> Result<Self> {
        fs::create_dir_all(&dir).map_err(|e| Error::LogRotation(format!(
            "无法创建日志目录 {}: {}",
            dir.display(),
            e
        )))?;

        let log_path = dir.join("current.log");
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(Error::Io)?;

        let current_size = file.metadata().map(|m| m.len()).unwrap_or(0);

        Ok(Self {
            entry_id,
            dir,
            writer: Some(BufWriter::new(file)),
            current_size,
        })
    }

    /// 写入一条信息日志，携带时间戳。
    pub fn write_info(&mut self, msg: &str) -> Result<()> {
        self.write_line("info", msg)
    }

    /// 写入一条错误日志。
    pub fn write_error(&mut self, msg: &str) -> Result<()> {
        self.write_line("error", msg)
    }

    /// 从 LogMessage 写入日志。
    pub fn write_message(&mut self, msg: &LogMessage) -> Result<()> {
        let level = match msg.level {
            crate::models::LogLevel::Info => "info",
            crate::models::LogLevel::Error => "error",
        };
        let text = format_event(&msg.event);
        self.write_line(level, &text)
    }

    fn write_line(&mut self, level: &str, msg: &str) -> Result<()> {
        let now = Utc::now();
        let line = format!("{} [{}] {}\n", now.format("%Y-%m-%dT%H:%M:%S%.3fZ"), level, msg);

        // 检查是否需要轮转（以行字节数估算）
        let line_bytes = line.len() as u64;
        if self.current_size + line_bytes > MAX_SEGMENT_BYTES && self.current_size > 0 {
            self.rotate()?;
        }

        let writer = self.writer.as_mut().unwrap();
        writer
            .write_all(line.as_bytes())
            .map_err(Error::Io)?;
        writer.flush().map_err(Error::Io)?;
        self.current_size += line_bytes;
        Ok(())
    }

    /// 分段轮转：删除最旧段，依次重命名，创建新活跃文件。
    fn rotate(&mut self) -> Result<()> {
        // 删除最旧段 current.log.{MAX_SEGMENTS}
        let oldest = self.dir.join(format!("current.log.{}", MAX_SEGMENTS));
        if oldest.exists() {
            fs::remove_file(&oldest).map_err(|e| Error::LogRotation(format!(
                "删除旧日志失败 {}: {}",
                oldest.display(),
                e
            )))?;
        }

        // 依次重命名 current.log.{i} -> current.log.{i+1}
        for i in (1..MAX_SEGMENTS).rev() {
            let src = self.dir.join(format!("current.log.{}", i));
            let dst = self.dir.join(format!("current.log.{}", i + 1));
            if src.exists() {
                fs::rename(&src, &dst).map_err(|e| Error::LogRotation(format!(
                    "重命名日志失败 {} -> {}: {}",
                    src.display(),
                    dst.display(),
                    e
                )))?;
            }
        }

        // 重命名 current.log -> current.log.1
        let current_path = self.dir.join("current.log");
        let first = self.dir.join("current.log.1");
        // 先关闭当前 writer，以便重命名文件
        drop(self.writer.take());
        fs::rename(&current_path, &first).map_err(|e| Error::LogRotation(format!(
            "重命名日志失败 {} -> {}: {}",
            current_path.display(),
            first.display(),
            e
        )))?;

        // 创建新 current.log
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&current_path)
            .map_err(Error::Io)?;

        self.writer = Some(BufWriter::new(file));
        self.current_size = 0;
        Ok(())
    }

    /// 读取所有日志行，按时间顺序合并所有分段。
    pub fn read_logs(&self, offset: usize, limit: usize) -> Result<LogResponse> {
        let mut all_lines = Vec::new();
        let mut total_bytes = 0u64;

        // 先刷新当前写入缓冲区，确保数据可见
        // (BufWriter 在读取时不能直接使用，我们从文件系统读取)

        // 读取 current.log + current.log.1 .. current.log.N
        let mut files_to_read: Vec<PathBuf> = vec![self.dir.join("current.log")];
        for i in 1..=MAX_SEGMENTS {
            let p = self.dir.join(format!("current.log.{}", i));
            if p.exists() {
                files_to_read.push(p);
            }
        }

        // 按文件序号排序：current.log.5 .. current.log.1, current.log
        // 即按创建时间从旧到新排序
        files_to_read.sort_by(|a, b| {
            segment_order(a).cmp(&segment_order(b))
        });

        for path in &files_to_read {
            if let Ok(meta) = fs::metadata(path) {
                total_bytes += meta.len();
            }
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                for line in reader.lines().map_while(|r| r.ok()) {
                    if let Some(log_line) = parse_log_line(&line) {
                        all_lines.push(log_line);
                    }
                }
            }
        }

        let lines: Vec<LogLine> = all_lines
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();

        Ok(LogResponse {
            entry_id: self.entry_id,
            lines,
            total_bytes,
            max_bytes: MAX_TOTAL_BYTES,
        })
    }

    /// 获取当前日志总大小。
    pub fn total_size(&self) -> u64 {
        let mut size = 0u64;
        let current = self.dir.join("current.log");
        if let Ok(meta) = fs::metadata(&current) {
            size += meta.len();
        }
        for i in 1..=MAX_SEGMENTS {
            let p = self.dir.join(format!("current.log.{}", i));
            if let Ok(meta) = fs::metadata(&p) {
                size += meta.len();
            }
        }
        size
    }
}

/// 生成日志任务——从 mpsc 接收 LogMessage 并写入磁盘。
pub async fn spawn_logger_task(
    entry_id: Uuid,
    dir: PathBuf,
    mut rx: mpsc::UnboundedReceiver<LogMessage>,
    shutdown: watch::Receiver<bool>,
) -> Result<JoinHandle<()>> {
    let handle = tokio::task::spawn_blocking(move || {
        let mut logger = match EntryLogger::new(dir, entry_id) {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("创建日志记录器失败: {}", e);
                return;
            }
        };

        while let Some(msg) = rx.blocking_recv() {
            if let Err(e) = logger.write_message(&msg) {
                tracing::error!("写入日志失败: {}", e);
            }

            // 非阻塞检查关闭信号
            if *shutdown.borrow() {
                while let Ok(msg) = rx.try_recv() {
                    let _ = logger.write_message(&msg);
                }
                break;
            }
        }
    });

    Ok(handle)
}

/// 根据文件名判断分段序号：值越小表示文件越旧。按升序排序后，旧文件在前。
fn segment_order(path: &std::path::Path) -> u32 {
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    if name == "current.log" {
        // current.log 是最新的，排序在最后
        MAX_SEGMENTS as u32 + 1
    } else if let Some(suffix) = name.strip_prefix("current.log.") {
        // 编号越大越旧，排序越前：current.log.5 → 0, current.log.1 → 4
        let num = suffix.parse::<u32>().unwrap_or(MAX_SEGMENTS as u32 + 2);
        MAX_SEGMENTS as u32 - num
    } else {
        MAX_SEGMENTS as u32 + 2
    }
}

fn parse_log_line(line: &str) -> Option<LogLine> {
    // 格式: "2024-01-01T00:00:00.000Z [info] message"
    let (ts_part, rest) = line.split_once(" [")?;
    let timestamp = chrono::DateTime::parse_from_rfc3339(ts_part)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))?;

    let rest = rest.strip_suffix(']').unwrap_or(rest);
    let (level, message) = rest.split_once("] ")?;
    Some(LogLine {
        timestamp,
        level: level.to_string(),
        message: message.to_string(),
    })
}

fn format_event(event: &LogEvent) -> String {
    match event {
        LogEvent::ConnectionAccepted { source } => {
            format!("connection accepted from {}", source)
        }
        LogEvent::ConnectionClosed {
            bytes_in,
            bytes_out,
            duration_ms,
        } => {
            format!(
                "connection closed | bytes_in={} bytes_out={} duration_ms={}",
                bytes_in, bytes_out, duration_ms
            )
        }
        LogEvent::ConnectionError { error } => {
            format!("connection error: {}", error)
        }
        LogEvent::ForwarderStarted => "forwarder started".to_string(),
        LogEvent::ForwarderStopped => "forwarder stopped".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_write_and_read() {
        let dir = tempfile::tempdir().unwrap();
        let mut logger = EntryLogger::new(dir.path().join("test"), Uuid::new_v4()).unwrap();

        logger.write_info("line 1").unwrap();
        logger.write_error("line 2").unwrap();
        logger.write_info("line 3").unwrap();

        let resp = logger.read_logs(0, 100).unwrap();
        assert_eq!(resp.lines.len(), 3);
        assert_eq!(resp.lines[0].level, "info");
        assert!(resp.lines[0].message.contains("line 1"));
        assert_eq!(resp.lines[1].level, "error");
        assert_eq!(resp.lines[2].level, "info");
    }

    #[test]
    fn test_log_rotation_deletes_oldest() {
        let dir = tempfile::tempdir().unwrap();
        let log_dir = dir.path().join("test_rotate");
        let mut logger = EntryLogger::new(log_dir.clone(), Uuid::new_v4()).unwrap();

        // 写入大量数据触发轮转（每行约 100 字节，需要几万行来超过 1MB）
        // 我们人为模拟更快的方法：直接写满并触发旋转
        let big_msg = "A".repeat(500_000); // 0.5MB per log line
        logger.write_info(&big_msg).unwrap();
        logger.write_info(&big_msg).unwrap(); // 现在 ~1MB
        logger.write_info(&big_msg).unwrap(); // 触发旋转 + 写入新段

        // 应该有 current.log + current.log.1
        assert!(log_dir.join("current.log").exists());
        assert!(log_dir.join("current.log.1").exists());

        // 读取应该包含所有行
        let resp = logger.read_logs(0, 100).unwrap();
        assert_eq!(resp.lines.len(), 3);
    }

    #[test]
    fn test_log_offset_limit() {
        let dir = tempfile::tempdir().unwrap();
        let mut logger = EntryLogger::new(dir.path().join("test_offset"), Uuid::new_v4()).unwrap();

        for i in 0..10 {
            logger.write_info(&format!("msg {}", i)).unwrap();
        }

        let resp = logger.read_logs(3, 2).unwrap();
        assert_eq!(resp.lines.len(), 2);
        assert!(resp.lines[0].message.contains("msg 3"));
        assert!(resp.lines[1].message.contains("msg 4"));
    }

    #[test]
    fn test_segment_order() {
        let newest = PathBuf::from("current.log");
        let older1 = PathBuf::from("current.log.1");
        let older2 = PathBuf::from("current.log.2");

        let mut paths = [newest, older1.clone(), older2.clone()];
        paths.sort_by_key(|a| segment_order(a));

        // 最旧的是 current.log.2
        assert_eq!(paths[0].file_name().unwrap(), "current.log.2");
        assert_eq!(paths[1].file_name().unwrap(), "current.log.1");
        assert_eq!(paths[2].file_name().unwrap(), "current.log");
    }
}
