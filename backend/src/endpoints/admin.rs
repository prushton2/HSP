use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::database;

pub async fn get_student_info(State(db_mutex): State<Arc<Mutex<dyn database::Database>>>) -> String {
    let mut db = db_mutex.lock().await;

    let all_tables = db.get_student_tables().await.unwrap();

    format!("{{\"student_info\":{},\"residencies\":{},\"student_activities\":{},\"activities\":{}}}",
        serde_json::to_string(&all_tables.0).unwrap(),
        serde_json::to_string(&all_tables.1).unwrap(),
        serde_json::to_string(&all_tables.2).unwrap(),
        serde_json::to_string(&all_tables.3).unwrap()
    )
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub fname: String,
    pub lname: String,
    pub number: i32,
    pub hall: String,
    pub room: i32,
    pub wing: String,
    pub role: String,
}
pub async fn new_sudent(State(db_mutex): State<Arc<Mutex<dyn database::Database>>>, Json(payload): Json<CreateUser>, ) -> (StatusCode, String) {
    let mut db = db_mutex.lock().await;
    
    let response = match db.create_student(&payload).await {
        Ok(_) => (StatusCode::CREATED, "".to_string()),
        Err(t) => (StatusCode::BAD_REQUEST, format!("{:?}", t))
    };

    return response;
}