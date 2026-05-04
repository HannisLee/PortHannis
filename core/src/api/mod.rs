mod entries;
mod control;
mod logs;

use std::sync::Arc;

use axum::{
    Json, Router,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;
use crate::manager::ForwardingManager;

/// 共享应用状态
#[derive(Clone)]
pub struct AppState {
    pub manager: Arc<ForwardingManager>,
}

/// 构建 Axum 路由（供 server 和 gui 共用）
pub fn build_router(state: AppState) -> Router {
    let api = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/entries", get(entries::list_entries).post(entries::create_entry))
        .route(
            "/api/entries/{id}",
            get(entries::get_entry)
                .put(entries::update_entry)
                .delete(entries::delete_entry),
        )
        .route("/api/entries/{id}/start", post(control::start_entry))
        .route("/api/entries/{id}/stop", post(control::stop_entry))
        .route("/api/entries/{id}/status", get(control::entry_status))
        .route("/api/entries/{id}/logs", get(logs::get_logs))
        .with_state(state);

    Router::new()
        .merge(api)
        .layer(CorsLayer::permissive())
}

/// 健康检查
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "ok": true }))
}
