use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::database::Error;
use crate::repository::auth_repository::FullUser;
use crate::types::Role;

#[derive(Deserialize)]
pub struct CreateUser {
    pub fname: String,
    pub lname: String,
    pub role: String,
    pub device: String
}

pub async fn create_user(State(state): State<Arc<super::Services>>, Json(body): Json<CreateUser>) -> (StatusCode, String) {
    let mut service = state.auth.lock().await;

    let new_user = FullUser {
        uuid: String::from(""),
        fname: body.fname,
        lname: body.lname,
        role: Role::from(body.role)
    };

    let token = match service.create_user(&new_user, &body.device).await {
        Ok(t) => t,
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, String::from(Error::ErrorDuring("Creating User".to_owned(), Box::new(t))))
    };

    (StatusCode::OK, format!("{{token: \"{}\"}}", token))
}

