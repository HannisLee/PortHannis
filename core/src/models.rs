use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type EntryId = Uuid;

/// 端口转发条目运行状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EntryStatus {
    Running,
    Stopped,
    Error { message: String },
}

/// 一个持久化的端口转发条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardingEntry {
    pub id: EntryId,
    pub name: String,
    /// 监听地址，如 "0.0.0.0"
    pub source_address: String,
    /// 监听端口
    pub source_port: u16,
    /// 转发目标地址
    pub target_address: String,
    /// 转发目标端口
    pub target_port: u16,
    /// 日志目录路径
    pub log_directory: String,
    /// 是否启用
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建或更新条目的请求体
#[derive(Debug, Deserialize)]
pub struct EntryRequest {
    pub name: String,
    #[serde(default = "default_source_address")]
    pub source_address: String,
    pub source_port: u16,
    pub target_address: String,
    pub target_port: u16,
    #[serde(default)]
    pub log_directory: Option<String>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_source_address() -> String {
    "0.0.0.0".to_string()
}

fn default_enabled() -> bool {
    true
}

/// 日志查询响应
#[derive(Debug, Serialize)]
pub struct LogResponse {
    pub entry_id: EntryId,
    pub lines: Vec<LogLine>,
    /// 当前日志总大小（字节）
    pub total_bytes: u64,
    /// 配置的日志上限（字节）
    pub max_bytes: u64,
}

/// 单条日志
#[derive(Debug, Clone, Serialize)]
pub struct LogLine {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
}

/// 日志级别
#[derive(Debug, Clone)]
pub enum LogLevel {
    Info,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Error => "error",
        }
    }
}

/// 日志事件类型
#[derive(Debug, Clone)]
pub enum LogEvent {
    ConnectionAccepted {
        source: std::net::SocketAddr,
    },
    ConnectionClosed {
        bytes_in: u64,
        bytes_out: u64,
        duration_ms: u64,
    },
    ConnectionError {
        error: String,
    },
    ForwarderStarted,
    ForwarderStopped,
}

/// 通过 mpsc 发送给日志任务的日志消息
#[derive(Debug, Clone)]
pub struct LogMessage {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub connection_id: Uuid,
    pub event: LogEvent,
}

/// 磁盘上的配置文件格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub entries: Vec<ForwardingEntry>,
}
