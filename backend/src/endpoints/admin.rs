use std::sync::Arc;

use axum::{extract::State, http::StatusCode};



pub async fn get_all_students(State(state): State<Arc<super::Services>>) -> (StatusCode, String) {
    let mut service = state.admin.lock().await;

    let tables = match service.get_all_tables().await {
        Ok(t) => t,
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, String::from(t))
    };

    (StatusCode::OK, serde_json::to_string(&tables).unwrap())
}