use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::{mpsc, watch, RwLock};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::config::ConfigStore;
use crate::error::{Error, Result};
use crate::forwarder::TcpProxy;
use crate::logger;
use crate::models::{EntryRequest, EntryStatus, ForwardingEntry, LogMessage, LogResponse};

/// 已注册代理的句柄
struct ProxyHandle {
    cancel: CancellationToken,
    join_handle: JoinHandle<()>,
}

/// 日志任务句柄
struct LoggerHandle {
    shutdown_tx: watch::Sender<bool>,
    join_handle: JoinHandle<()>,
    #[allow(dead_code)]
    log_tx: mpsc::UnboundedSender<LogMessage>,
}

/// 端口转发管理器——系统的核心调度器，串联配置、转发、日志三者。
pub struct ForwardingManager {
    config_store: Arc<RwLock<ConfigStore>>,
    proxies: RwLock<HashMap<Uuid, ProxyHandle>>,
    loggers: RwLock<HashMap<Uuid, LoggerHandle>>,
}

impl ForwardingManager {
    /// 创建管理器并加载配置。
    pub async fn new(config_path: PathBuf) -> Result<Self> {
        let config_store = ConfigStore::load(config_path)?;
        info!("配置已加载: {} 条目", config_store.entries().len());

        Ok(Self {
            config_store: Arc::new(RwLock::new(config_store)),
            proxies: RwLock::new(HashMap::new()),
            loggers: RwLock::new(HashMap::new()),
        })
    }

    // ─── CRUD ────────────────────────────────────────────────────

    pub async fn list_entries(&self) -> Vec<ForwardingEntry> {
        self.config_store.read().await.entries_cloned()
    }

    pub async fn get_entry(&self, id: Uuid) -> Result<ForwardingEntry> {
        self.config_store.read().await.find_entry(id).cloned()
    }

    pub async fn create_entry(&self, req: EntryRequest) -> Result<ForwardingEntry> {
        self.config_store.write().await.add_entry(req)
    }

    pub async fn update_entry(&self, id: Uuid, req: EntryRequest) -> Result<ForwardingEntry> {
        // 先停止正在运行的转发
        let running = {
            let proxies = self.proxies.read().await;
            proxies.contains_key(&id)
        };
        if running {
            self.stop_forwarding(id).await?;
        }

        let entry = self.config_store.write().await.update_entry(id, req)?;

        // 如果更新后 enabled 为 true，自动重启
        if entry.enabled {
            if let Err(e) = self.start_forwarding(id).await {
                warn!("更新后自动重启失败 ({}): {}", id, e);
            }
        }

        Ok(entry)
    }

    pub async fn delete_entry(&self, id: Uuid, cleanup_logs: bool) -> Result<()> {
        // 确保已停止
        let running = {
            let proxies = self.proxies.read().await;
            proxies.contains_key(&id)
        };
        if running {
            self.stop_forwarding(id).await?;
        }

        let entry = self.config_store.write().await.remove_entry(id)?;

        // 清理日志文件
        if cleanup_logs {
            let log_dir = PathBuf::from(&entry.log_directory);
            if log_dir.exists() {
                if let Err(e) = std::fs::remove_dir_all(&log_dir) {
                    error!("清理日志目录失败 {}: {}", log_dir.display(), e);
                }
            }
        }

        Ok(())
    }

    // ─── 生命周期 ────────────────────────────────────────────────

    /// 启动指定条目的端口转发。
    pub async fn start_forwarding(&self, id: Uuid) -> Result<EntryStatus> {
        // 检查是否已在运行
        {
            let proxies = self.proxies.read().await;
            if proxies.contains_key(&id) {
                return Err(Error::InvalidState {
                    id,
                    status: "running".into(),
                });
            }
        }

        let entry = self.config_store.read().await.find_entry(id).cloned()?;

        let source_addr: SocketAddr = format!("{}:{}", entry.source_address, entry.source_port)
            .parse()
            .map_err(|e| Error::Validation(format!("无效的源地址: {}", e)))?;

        let target_addr: SocketAddr = format!("{}:{}", entry.target_address, entry.target_port)
            .parse()
            .map_err(|e| Error::Validation(format!("无效的目标地址: {}", e)))?;

        // 创建日志通道并启动日志任务
        let log_dir = PathBuf::from(&entry.log_directory);
        let (log_tx, log_rx) = mpsc::unbounded_channel();
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let logger_handle = logger::spawn_logger_task(id, log_dir, log_rx, shutdown_rx).await?;

        let logger_handle = LoggerHandle {
            shutdown_tx,
            join_handle: logger_handle,
            log_tx: log_tx.clone(),
        };

        // 创建取消令牌并启动代理
        let cancel = CancellationToken::new();
        let proxy = TcpProxy::new(id, source_addr, target_addr, log_tx, cancel.clone());

        let join_handle = tokio::spawn(async move {
            if let Err(e) = proxy.run().await {
                error!("代理运行失败 ({}): {}", id, e);
            }
        });

        let proxy_handle = ProxyHandle {
            cancel,
            join_handle,
        };

        // 存储句柄
        {
            self.proxies.write().await.insert(id, proxy_handle);
            self.loggers.write().await.insert(id, logger_handle);
        }

        info!("转发已启动: {} -> {}", source_addr, target_addr);
        Ok(EntryStatus::Running)
    }

    /// 停止指定条目的端口转发。
    pub async fn stop_forwarding(&self, id: Uuid) -> Result<EntryStatus> {
        // 取消代理
        let proxy_handle = {
            let mut proxies = self.proxies.write().await;
            proxies.remove(&id)
        };

        match proxy_handle {
            Some(handle) => {
                handle.cancel.cancel();

                // 等待代理任务结束（最多等 5 秒）
                let _ = tokio::time::timeout(
                    std::time::Duration::from_secs(5),
                    handle.join_handle,
                )
                .await;
            }
            None => {
                return Err(Error::InvalidState {
                    id,
                    status: "stopped".into(),
                });
            }
        }

        // 关闭日志任务
        if let Some(logger_handle) = self.loggers.write().await.remove(&id) {
            let _ = logger_handle.shutdown_tx.send(true);
            let _ = tokio::time::timeout(
                std::time::Duration::from_secs(3),
                logger_handle.join_handle,
            )
            .await;
        }

        info!("转发已停止: {}", id);
        Ok(EntryStatus::Stopped)
    }

    /// 获取条目当前运行状态。
    pub async fn get_status(&self, id: Uuid) -> Result<EntryStatus> {
        // 确保条目存在
        self.config_store.read().await.find_entry(id)?;

        let proxies = self.proxies.read().await;
        if proxies.contains_key(&id) {
            Ok(EntryStatus::Running)
        } else {
            Ok(EntryStatus::Stopped)
        }
    }

    // ─── 日志 ────────────────────────────────────────────────────

    /// 获取指定条目的日志（分页）。
    pub async fn get_logs(&self, id: Uuid, offset: usize, limit: usize) -> Result<LogResponse> {
        // 确保条目存在
        let entry = self.config_store.read().await.find_entry(id).cloned()?;

        let log_dir = PathBuf::from(&entry.log_directory);
        let entry_logger = logger::EntryLogger::new(log_dir, id)?;
        entry_logger.read_logs(offset, limit)
    }

    /// 获取所有条目的状态快照（用于 Dashboard）。
    pub async fn get_all_status(&self) -> HashMap<Uuid, EntryStatus> {
        let entries = self.config_store.read().await.entries_cloned();
        let proxies = self.proxies.read().await;

        entries
            .into_iter()
            .map(|e| {
                let status = if proxies.contains_key(&e.id) {
                    EntryStatus::Running
                } else {
                    EntryStatus::Stopped
                };
                (e.id, status)
            })
            .collect()
    }

    /// 停止所有正在运行的转发（用于优雅关闭）。
    pub async fn shutdown_all(&self) {
        let ids: Vec<Uuid> = {
            self.proxies.read().await.keys().cloned().collect()
        };

        for id in ids {
            if let Err(e) = self.stop_forwarding(id).await {
                error!("关闭转发失败 ({}): {}", id, e);
            }
        }

        info!("所有转发已关闭");
    }

    /// 自动启动配置中标记为 enabled 的条目。
    pub async fn auto_start_enabled(&self) {
        let entries = self.config_store.read().await.entries_cloned();
        for entry in entries {
            if entry.enabled {
                match self.start_forwarding(entry.id).await {
                    Ok(_) => info!("自动启动: {} ({})", entry.name, entry.id),
                    Err(e) => warn!("自动启动失败 {}: {}", entry.name, e),
                }
            }
        }
    }
}
