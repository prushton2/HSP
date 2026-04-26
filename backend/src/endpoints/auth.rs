use std::sync::Arc;

use axum::http::HeaderValue;
use axum::http::header::SET_COOKIE;
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

use crate::database::Error;
use crate::repository::auth_repository::{FullUser, UpdateUser};
use crate::types::Role;

#[derive(Deserialize)]
pub struct CreateUser {
    pub fname: String,
    pub lname: String,
    pub role: String,
}

pub async fn create_user(State(state): State<Arc<super::Services>>, Json(body): Json<CreateUser>) -> (StatusCode, String) {
    let mut service = state.auth.lock().await;

    let new_user = FullUser {
        uuid: String::from(""),
        fname: body.fname,
        lname: body.lname,
        role: Role::from(body.role)
    };

    let token = match service.create_user(&new_user).await {
        Ok(t) => t,
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, String::from(Error::ErrorDuring("Creating User".to_owned(), Box::new(t))))
    };

    (StatusCode::OK, format!("{{\"token\": \"{}\"}}", token))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub signup_hash: String
}
pub async fn signup(State(state): State<Arc<super::Services>>, Json(body): Json<LoginRequest>) -> impl IntoResponse {
    let mut service = state.auth.lock().await;

    let token = match service.signup(&body.signup_hash).await {
        Ok(t) => t,
        Err(t) => return (StatusCode::BAD_REQUEST, String::from(t)).into_response()
    };


    let cookie = format!("token={}; HttpOnly; SameSite=Strict; Path=/", token);

    (
        StatusCode::OK,
        [(SET_COOKIE, HeaderValue::from_str(cookie.as_str()).unwrap())],
        String::from("")
    ).into_response()
}

#[derive(Deserialize)]
pub struct HttpUpdateUser {
    pub uuid: String,
    pub field: String,
    pub str_field: String,
}
pub async fn update_user(State(state): State<Arc<super::Services>>, Json(body): Json<HttpUpdateUser>) -> (StatusCode, String) {
    let mut update = UpdateUser {
        fname: None,
        lname: None,
        role: None
    };

    match body.field.as_str() {
        "first name" => update.fname = Some(body.str_field),
        "last name"  => update.lname = Some(body.str_field),
        "role"       => update.role  = Some(Role::from(body.str_field)),
        _            => return (StatusCode::BAD_REQUEST, "Invalid Field".to_string())
    }

    let mut service = state.auth.lock().await;

    match service.update_user(&body.uuid, &update).await {
        Ok(_) => (StatusCode::OK, String::from("")),
        Err(t) => (StatusCode::INTERNAL_SERVER_ERROR, String::from(t))
    }
}