use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::{body::Body, http::{Response, Uri}, routing::get};
use clap::Parser;
use rust_embed::RustEmbed;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::info;

use porthannis_core::api::{self, AppState};
use porthannis_core::manager::ForwardingManager;

/// PortHannis — 轻量级端口转发管理器
#[derive(Parser, Debug)]
#[command(name = "porthannis", version, about)]
struct Cli {
    /// JSON 配置文件路径
    #[arg(short, long, env = "PORTHANNIS_CONFIG")]
    config: Option<PathBuf>,

    /// API 绑定地址
    #[arg(short, long, default_value = "127.0.0.1", env = "PORTHANNIS_BIND")]
    bind: String,

    /// API 绑定端口
    #[arg(short = 'P', long, default_value = "25879", env = "PORTHANNIS_PORT")]
    port: u16,
}

/// 嵌入前端构建产物（frontend/dist/）
#[derive(RustEmbed)]
#[folder = "../frontend/dist/"]
struct FrontendAssets;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "porthannis=info,tower_http=info".into()),
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

    info!("配置路径: {}", config_path.display());

    let manager = Arc::new(ForwardingManager::new(config_path).await?);
    manager.auto_start_enabled().await;

    let app_state = AppState {
        manager: manager.clone(),
    };

    let app = api::build_router(app_state)
        .route("/{*path}", get(serve_frontend))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", cli.bind, cli.port).parse()?;
    info!("PortHannis 已启动: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            signal::ctrl_c().await.ok();
            info!("收到终止信号，正在关闭所有转发...");
            manager.shutdown_all().await;
        })
        .await?;

    Ok(())
}

/// 服务嵌入式前端静态文件，支持 SPA fallback。
async fn serve_frontend(uri: Uri) -> Response<Body> {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match FrontendAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header("Content-Type", mime.as_ref())
                .body(Body::from(content.data))
                .unwrap()
        }
        None => {
            // SPA fallback: 返回 index.html
            let content = FrontendAssets::get("index.html").unwrap();
            Response::builder()
                .header("Content-Type", "text/html")
                .body(Body::from(content.data))
                .unwrap()
        }
    }
}
