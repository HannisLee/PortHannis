// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tracing::info;

use porthannis_core::api::{self, AppState};
use porthannis_core::manager::ForwardingManager;
use porthannis_gui_lib::run_tauri;

#[derive(Parser, Debug)]
#[command(name = "porthannis-gui", version, about)]
struct Cli {
    #[arg(short, long, env = "PORTHANNIS_CONFIG")]
    config: Option<PathBuf>,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "porthannis=info".into()),
        )
        .init();

    let cli = Cli::parse();

    let config_path = cli.config.unwrap_or_else(|| {
        let data_dir = directories::ProjectDirs::from("com", "porthannis", "PortHannis")
            .map(|d| d.config_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        std::fs::create_dir_all(&data_dir).ok();
        data_dir.join("config.json")
    });

    let api_port = find_available_port(25879).unwrap_or(25880);
    let (ready_tx, ready_rx) = oneshot::channel();

    // 后台线程：启动 Axum 服务器
    let config_path_clone = config_path.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        rt.block_on(async move {
            let manager = ForwardingManager::new(config_path_clone)
                .await
                .expect("创建管理器失败");
            manager.auto_start_enabled().await;

            let app_state = AppState {
                manager: Arc::new(manager),
            };

            let app = api::build_router(app_state);
            let addr: SocketAddr = format!("127.0.0.1:{}", api_port)
                .parse()
                .expect("无效的 API 地址");

            let listener = TcpListener::bind(addr).await.expect("绑定 API 端口失败");
            info!("后端 API 已启动: http://{}", addr);

            // 通知主线程服务器已就绪
            let _ = ready_tx.send(());

            axum::serve(listener, app).await.ok();
        });
    });

    // 等待后台服务就绪（最多等 10 秒）
    let _ = ready_rx.blocking_recv();
    std::thread::sleep(std::time::Duration::from_millis(200));

    // 启动 Tauri GUI
    run_tauri(api_port);
}

fn find_available_port(start: u16) -> Option<u16> {
    for port in start..start + 100 {
        let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().ok()?;
        if std::net::TcpListener::bind(addr).is_ok() {
            return Some(port);
        }
    }
    None
}
