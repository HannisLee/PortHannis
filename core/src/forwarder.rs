use std::net::SocketAddr;
use std::time::Instant;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{LogEvent, LogLevel, LogMessage};
/// TCP 代理——监听 source_addr 并转发连接到 target_addr。
pub struct TcpProxy {
    entry_id: Uuid,
    source_addr: SocketAddr,
    target_addr: SocketAddr,
    log_tx: mpsc::UnboundedSender<LogMessage>,
    cancel: CancellationToken,
}

impl TcpProxy {
    pub fn new(
        entry_id: Uuid,
        source_addr: SocketAddr,
        target_addr: SocketAddr,
        log_tx: mpsc::UnboundedSender<LogMessage>,
        cancel: CancellationToken,
    ) -> Self {
        Self {
            entry_id,
            source_addr,
            target_addr,
            log_tx,
            cancel,
        }
    }

    /// 运行代理主循环。返回时代表被取消或发生致命错误。
    pub async fn run(self) -> Result<()> {
        let listener = TcpListener::bind(self.source_addr).await.map_err(|e| {
            Error::BindError {
                addr: self.source_addr,
                source: e,
            }
        })?;

        info!(
            "端口转发启动: {} (entry {}) -> {}",
            self.source_addr, self.entry_id, self.target_addr
        );

        send_log(
            &self.log_tx,
            Uuid::nil(),
            LogLevel::Info,
            LogEvent::ForwarderStarted,
        );

        loop {
            tokio::select! {
                _ = self.cancel.cancelled() => break,
                result = listener.accept() => {
                    match result {
                        Ok((inbound, src_addr)) => {
                            let conn_id = Uuid::new_v4();
                            let target = self.target_addr;
                            let cancel = self.cancel.clone();
                            let log_tx = self.log_tx.clone();

                            tokio::spawn(async move {
                                handle_connection(inbound, src_addr, target, conn_id, cancel, log_tx).await;
                            });
                        }
                        Err(e) => {
                            error!("接受连接失败: {}", e);
                            send_log(
                                &self.log_tx,
                                Uuid::nil(),
                                LogLevel::Error,
                                LogEvent::ConnectionError {
                                    error: format!("accept failed: {}", e),
                                },
                            );
                        }
                    }
                }
            }
        }

        info!("端口转发停止: {} (entry {})", self.source_addr, self.entry_id);

        send_log(
            &self.log_tx,
            Uuid::nil(),
            LogLevel::Info,
            LogEvent::ForwarderStopped,
        );

        Ok(())
    }
}

/// 处理单个连接的转发逻辑。
async fn handle_connection(
    mut inbound: TcpStream,
    src_addr: SocketAddr,
    target: SocketAddr,
    conn_id: Uuid,
    cancel: CancellationToken,
    log_tx: mpsc::UnboundedSender<LogMessage>,
) {
    send_log(
        &log_tx,
        conn_id,
        LogLevel::Info,
        LogEvent::ConnectionAccepted { source: src_addr },
    );

    let mut outbound = match TcpStream::connect(target).await {
        Ok(s) => s,
        Err(e) => {
            send_log(
                &log_tx,
                conn_id,
                LogLevel::Error,
                LogEvent::ConnectionError {
                    error: e.to_string(),
                },
            );
            return;
        }
    };

    let start = Instant::now();

    // tokio::io::copy_bidirectional 需要 tokio 1.41+
    // 如果不可用，使用手动双向拷贝作为 fallback
    let result = tokio::select! {
        _ = cancel.cancelled() => {
            return;
        }
        r = copy_bidirectional(&mut inbound, &mut outbound) => r,
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    match result {
        Ok((bytes_in, bytes_out)) => {
            send_log(
                &log_tx,
                conn_id,
                LogLevel::Info,
                LogEvent::ConnectionClosed {
                    bytes_in,
                    bytes_out,
                    duration_ms,
                },
            );
        }
        Err(e) => {
            send_log(
                &log_tx,
                conn_id,
                LogLevel::Error,
                LogEvent::ConnectionError {
                    error: e.to_string(),
                },
            );
        }
    }
}

/// 双向数据拷贝。使用 tokio::io::copy_bidirectional 如果可用，
/// 否则回退到手动 tokio::join!(copy(a,b), copy(b,a))。
async fn copy_bidirectional(
    a: &mut TcpStream,
    b: &mut TcpStream,
) -> std::result::Result<(u64, u64), std::io::Error> {
    let (mut a_read, mut a_write) = a.split();
    let (mut b_read, mut b_write) = b.split();

    let a_to_b = tokio::io::copy(&mut a_read, &mut b_write);
    let b_to_a = tokio::io::copy(&mut b_read, &mut a_write);

    let (ab, ba) = tokio::try_join!(a_to_b, b_to_a)?;
    Ok((ab, ba))
}

fn send_log(
    tx: &mpsc::UnboundedSender<LogMessage>,
    connection_id: Uuid,
    level: LogLevel,
    event: LogEvent,
) {
    let msg = LogMessage {
        timestamp: chrono::Utc::now(),
        level,
        connection_id,
        event,
    };
    if tx.send(msg).is_err() {
        // 接收端已关闭，忽略
    }
}
