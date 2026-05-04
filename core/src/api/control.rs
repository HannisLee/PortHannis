use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::AppState;

pub async fn start_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match state.manager.start_forwarding(id).await {
        Ok(status) => Ok(Json(serde_json::json!(status))),
        Err(e) => {
            let status = map_error_status(&e);
            Err((
                status,
                Json(serde_json::json!({"error": "start_failed", "message": e.to_string()})),
            ))
        }
    }
}

pub async fn stop_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match state.manager.stop_forwarding(id).await {
        Ok(status) => Ok(Json(serde_json::json!(status))),
        Err(e) => {
            let status = map_error_status(&e);
            Err((
                status,
                Json(serde_json::json!({"error": "stop_failed", "message": e.to_string()})),
            ))
        }
    }
}

pub async fn entry_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match state.manager.get_status(id).await {
        Ok(status) => Ok(Json(serde_json::json!(status))),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "not_found", "message": e.to_string()})),
        )),
    }
}

fn map_error_status(e: &crate::error::Error) -> StatusCode {
    use crate::error::Error;
    match e {
        Error::NotFound(_) => StatusCode::NOT_FOUND,
        Error::InvalidState { .. } => StatusCode::CONFLICT,
        Error::PortInUse(_) => StatusCode::CONFLICT,
        Error::BindError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
