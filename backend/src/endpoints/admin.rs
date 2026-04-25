use std::sync::Arc;

use axum::{extract::State, http::StatusCode};



pub async fn get_all_students(State(state): State<Arc<super::Services>>) -> (StatusCode, String) {
    // let mut db = db_mutex.lock().await;

    // let all_tables = db.get_student_tables().await.unwrap();

    // format!("{{\"student_info\":{},\"residencies\":{},\"student_activities\":{},\"activities\":{}}}",
    //     serde_json::to_string(&all_tables.0).unwrap(),
    //     serde_json::to_string(&all_tables.1).unwrap(),
    //     serde_json::to_string(&all_tables.2).unwrap(),
    //     serde_json::to_string(&all_tables.3).unwrap()
    // )

    let mut service = state.admin.lock().await;

    let tables = match service.get_all_tables().await {
        Ok(t) => t,
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, "String::from(t)".to_string())
    };

    (StatusCode::OK, serde_json::to_string(&tables).unwrap())
}