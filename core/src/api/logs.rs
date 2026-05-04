use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use super::AppState;

#[derive(Deserialize)]
pub struct LogQuery {
    #[serde(default)]
    pub offset: usize,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    500
}

pub async fn get_logs(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<LogQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match state.manager.get_logs(id, params.offset, params.limit).await {
        Ok(response) => Ok(Json(serde_json::json!(response))),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "not_found", "message": e.to_string()})),
        )),
    }
}
