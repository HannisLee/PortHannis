// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;
use tokio::net::TcpListener;
use tracing::info;

use porthannis_core::api::{self, AppState};
use porthannis_core::manager::ForwardingManager;
use porthannis_gui_lib::run_tauri;

#[derive(Parser, Debug)]
#[command(name = "porthannis-gui", version, about)]
struct Cli {
    /// JSON 配置文件路径
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

    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    // 查找可用端口（从 25879 开始，如果被占用则递增）
    let api_port = find_available_port(25879).unwrap_or(25880);

    // 在后台线程启动 Axum 服务器
    let config_path_clone = config_path.clone();
    std::thread::spawn(move || {
        rt.block_on(async move {
            let manager = ForwardingManager::new(config_path_clone)
                .await
                .expect("创建管理器失败");
            manager.auto_start_enabled().await;

            let app_state = AppState {
                manager: std::sync::Arc::new(manager),
            };

            let app = api::build_router(app_state);

            let addr: SocketAddr = format!("127.0.0.1:{}", api_port)
                .parse()
                .expect("无效的 API 地址");

            info!("后端 API 已启动: http://{}", addr);

            let listener = TcpListener::bind(addr).await.expect("绑定 API 端口失败");
            axum::serve(listener, app).await.ok();
        });
    });

    // 启动 Tauri
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
