use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::EntryRequest;
use super::AppState;

#[derive(Deserialize)]
pub struct DeleteParams {
    #[serde(default)]
    pub cleanup_logs: bool,
}

pub async fn list_entries(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let entries = state.manager.list_entries().await;
    Ok(Json(serde_json::json!(entries)))
}

pub async fn create_entry(
    State(state): State<AppState>,
    Json(req): Json<EntryRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    if let Err(e) = validate_entry_request(&req) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "validation", "message": e})),
        ));
    }

    match state.manager.create_entry(req).await {
        Ok(entry) => Ok((
            StatusCode::CREATED,
            Json(serde_json::json!(entry)),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "internal", "message": e.to_string()})),
        )),
    }
}

pub async fn get_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match state.manager.get_entry(id).await {
        Ok(entry) => Ok(Json(serde_json::json!(entry))),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "not_found", "message": e.to_string()})),
        )),
    }
}

pub async fn update_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<EntryRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if let Err(e) = validate_entry_request(&req) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "validation", "message": e})),
        ));
    }

    match state.manager.update_entry(id, req).await {
        Ok(entry) => Ok(Json(serde_json::json!(entry))),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "not_found", "message": e.to_string()})),
        )),
    }
}

pub async fn delete_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<DeleteParams>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    match state.manager.delete_entry(id, params.cleanup_logs).await {
        Ok(_) => Ok((
            StatusCode::NO_CONTENT,
            Json(serde_json::json!({})),
        )),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "not_found", "message": e.to_string()})),
        )),
    }
}

fn validate_entry_request(req: &EntryRequest) -> std::result::Result<(), String> {
    if req.name.trim().is_empty() {
        return Err("名称不能为空".into());
    }
    if req.source_port == 0 {
        return Err("源端口必须在 1-65535 之间".into());
    }
    if req.target_port == 0 {
        return Err("目标端口必须在 1-65535 之间".into());
    }
    if req.target_address.trim().is_empty() {
        return Err("目标地址不能为空".into());
    }
    Ok(())
}
