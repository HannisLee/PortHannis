use std::sync::Arc;

use axum::{
    body::Body,
    http::{Response, Uri},
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tracing::info;

#[path = "../core.rs"]
mod core;

use core::{AppState, ProxyManager};

const INDEX_HTML: &str = include_str!("../web.html");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "porthannis=info,tower_http=info".into()),
        )
        .init();

    let manager = ProxyManager::new().await?;
    manager.auto_start_enabled().await;

    let app_state = AppState {
        manager: Arc::new(manager),
    };

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/entries", get(core::list_entries).post(core::create_entry))
        .route(
            "/api/entries/{id}",
            get(core::get_entry)
                .put(core::update_entry)
                .delete(core::delete_entry),
        )
        .route("/api/entries/{id}/start", post(core::start_entry))
        .route("/api/entries/{id}/stop", post(core::stop_entry))
        .route("/api/entries/{id}/status", get(core::get_entry_status))
        .route("/api/entries/{id}/logs", get(core::get_entry_logs))
        .fallback(serve_frontend)
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr = "127.0.0.1:7777";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("PortHannis 启动: http://{}", addr);

    let browser_url = format!("http://{}", addr);
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        if let Err(e) = opener::open(&browser_url) {
            tracing::debug!("自动打开浏览器失败: {}", e);
        }
    });

    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn serve_frontend(_uri: Uri) -> Response<Body> {
    Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(INDEX_HTML))
        .unwrap()
}
