// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    body::Body,
    http::{Response, Uri},
    routing::{get, post},
    Router,
};
use tokio::sync::oneshot;
use tower_http::cors::CorsLayer;
use tracing::info;

#[path = "../../server/core.rs"]
mod core;

use core::{AppState, ProxyManager};

use porthannis_gui_lib::run_tauri;

const INDEX_HTML: &str = include_str!("../../server/web.html");

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "porthannis=info".into()),
        )
        .init();

    let api_port = find_available_port(25879).unwrap_or(25880);
    let (ready_tx, ready_rx) = oneshot::channel();

    // Background thread: start Axum server
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        rt.block_on(async move {
            let manager = ProxyManager::new().await.expect("Failed to create ProxyManager");
            manager.auto_start_enabled().await;

            let app_state = AppState {
                manager: Arc::new(manager),
            };

            let app = build_router(app_state);
            let addr: SocketAddr = format!("127.0.0.1:{}", api_port)
                .parse()
                .expect("Invalid API address");

            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .expect("Failed to bind API port");
            info!("后端 API 已启动: http://{}", addr);

            let _ = ready_tx.send(());
            axum::serve(listener, app).await.ok();
        });
    });

    // Wait for backend to be ready
    let _ = ready_rx.blocking_recv();

    // Launch Tauri GUI
    run_tauri(api_port);
}

fn build_router(state: AppState) -> Router {
    Router::new()
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
        .with_state(state)
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

fn find_available_port(start: u16) -> Option<u16> {
    for port in start..start + 100 {
        let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().ok()?;
        if std::net::TcpListener::bind(addr).is_ok() {
            return Some(port);
        }
    }
    None
}
