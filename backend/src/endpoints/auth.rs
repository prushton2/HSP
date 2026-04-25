use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::database;

#[derive(Deserialize)]
pub struct CreateUser {
    pub fname: String,
    pub lname: String,
    pub role: String,
    pub device: String
}

pub async fn create_user(State(db_mutex): State<Arc<Mutex<dyn database::Database>>>, Json(body): Json<CreateUser>) -> (StatusCode, String) {
    let mut db = db_mutex.lock().await;
    
    match db.create_user(&body.fname, &body.lname, body.role.as_str().into(), &body.device).await {
        Ok(token) => (StatusCode::CREATED, format!("{{token: {}}}", token)),
        Err(t) => (StatusCode::BAD_REQUEST, format!("{:?}", t))
    }
}