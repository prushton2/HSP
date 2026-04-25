use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::database;



#[derive(Deserialize)]
pub struct CreateUser {
    pub fname: String,
    pub lname: String,
    pub pronouns: String,
    pub number: i32,
    pub hall: String,
    pub room: i32,
    pub wing: String,
    pub role: String,
}
pub async fn new_sudent(State(db_mutex): State<Arc<Mutex<dyn database::Database>>>, Json(body): Json<CreateUser>) -> (StatusCode, String) {
    let mut db = db_mutex.lock().await;
    
    let response = match db.create_student(&body).await {
        Ok(_) => (StatusCode::CREATED, "".to_string()),
        Err(t) => (StatusCode::BAD_REQUEST, format!("{:?}", t))
    };

    return response;
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct EditUser {
    pub uuid: String,
    pub field: String,
    pub str_field: String,
    pub int_field: i32,
}
pub async fn edit_sudent(State(db_mutex): State<Arc<Mutex<dyn database::Database>>>, Json(body): Json<EditUser>) -> (StatusCode, String) {
    let mut db = db_mutex.lock().await;
    
    let field = {
        if body.str_field != "" {
            database::FieldValue::Str(&body.str_field)
        } else {
            database::FieldValue::I32(body.int_field)
        }
    };

    let response = match db.edit_student(&body.uuid, &body.field, &field).await {
        Ok(_) => (StatusCode::CREATED, "".to_string()),
        Err(t) => (StatusCode::BAD_REQUEST, format!("{:?}", t))
    };

    return response;
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct GetStudent {
    pub uuid: String,
    pub decrypt: bool
}
pub async fn get_student(State(db_mutex): State<Arc<Mutex<dyn database::Database>>>, Json(body): Json<GetStudent>) -> (StatusCode, String) {
    let mut db = db_mutex.lock().await;
    
    let response = match db.get_student(&body.uuid, body.decrypt).await {
        Ok(t) => (StatusCode::OK, serde_json::to_string(&t).unwrap()),
        Err(t) => (StatusCode::BAD_REQUEST, format!("{:?}", t))
    };

    return response;
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DeleteStudent {
    pub uuid: String,
}
pub async fn delete_student(State(db_mutex): State<Arc<Mutex<dyn database::Database>>>, Json(body): Json<DeleteStudent>) -> (StatusCode, String) {
    let mut db = db_mutex.lock().await;
    
    let response = match db.delete_student(&body.uuid).await {
        Ok(t) => (StatusCode::OK, serde_json::to_string(&t).unwrap()),
        Err(t) => (StatusCode::BAD_REQUEST, format!("{:?}", t))
    };

    return response;
}