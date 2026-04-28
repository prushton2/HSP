use std::sync::Arc;

use axum::http::HeaderValue;
use axum::http::header::SET_COOKIE;
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::TOKEN_EXPIRY;
use crate::repository::auth_repository::{FullUser, UpdateUser};
use crate::types::{Role, Error};

#[derive(Deserialize)]
pub struct CreateUser {
    pub fname: String,
    pub lname: String,
    pub role: String,
}

pub async fn create_user(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<CreateUser>) -> (StatusCode, String) {
    let service = state.auth.read().await;
    let user = match service.is_authenticated(&jar, &Role::Owner, "create_user").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };

    let new_user = FullUser {
        uuid: String::from(""),
        fname: body.fname,
        lname: body.lname,
        role: Role::from(body.role)
    };

    let token = match service.create_user(&new_user).await {
        Ok(t) => t,
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, t.log_to_obfuscated(&user.uuid))
    };

    (StatusCode::OK, format!("{{\"token\": \"{}\"}}", token))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub signup_hash: String
}
pub async fn signup(State(state): State<Arc<super::Services>>, Json(body): Json<LoginRequest>) -> impl IntoResponse {
    let service = state.auth.read().await;

    let token = match service.signup(&body.signup_hash).await {
        Ok(t) => t,
        Err(t) => return (StatusCode::BAD_REQUEST, t.log_to_obfuscated("NO UUID")).into_response()
    };

    let cookie = format!("token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age={}", token, TOKEN_EXPIRY);

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
pub async fn update_user(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<HttpUpdateUser>) -> (StatusCode, String) {
    let service = state.auth.read().await;
    let user = match service.is_authenticated(&jar, &Role::Owner, "update_user").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };

    let mut update = UpdateUser {
        fname: None,
        lname: None,
        role: None
    };

    match body.field.as_str() {
        "first name" => update.fname = Some(body.str_field),
        "last name"  => update.lname = Some(body.str_field),
        "role"       => update.role  = Some(Role::from(body.str_field)),
        t            => return (StatusCode::BAD_REQUEST, Error::InvalidParameter("field".to_string(), t.to_string()).log_to_obfuscated(&user.uuid))
    }


    match service.update_user(&body.uuid, &update).await {
        Ok(_) => (StatusCode::OK, String::from("")),
        Err(t) => (StatusCode::INTERNAL_SERVER_ERROR, t.log_to_obfuscated(&user.uuid))
    }
}

#[derive(Deserialize)]
pub struct HttpDeleteUser {
    uuid: String
}
pub async fn delete_user(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<HttpDeleteUser>) -> (StatusCode, String) {
    let service = state.auth.read().await;
    let user = match service.is_authenticated(&jar, &Role::Owner, "delete_user").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };

    match service.delete_user(body.uuid.as_str()).await {
        Ok(_) => (StatusCode::OK, String::from("")),
        Err(t) => (StatusCode::BAD_REQUEST, t.log_to_obfuscated(&user.uuid))
    }
}

#[derive(Deserialize)]
pub struct HttpRevokeTokens {
    uuid: String
}
pub async fn revoke_tokens(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<HttpRevokeTokens>) -> (StatusCode, String) {
    let service = state.auth.read().await;
    let user = match service.is_authenticated(&jar, &Role::Owner, "revoke_tokens").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };

    match service.revoke_tokens(body.uuid.as_str()).await {
        Ok(_) => (StatusCode::OK, String::from("")),
        Err(t) => (StatusCode::BAD_REQUEST, t.log_to_obfuscated(&user.uuid))
    }
}

#[derive(Deserialize)]
pub struct HttpGrantToken {
    uuid: String
}
pub async fn grant_token(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<HttpGrantToken>) -> (StatusCode, String) {
    let service = state.auth.read().await;
    let user = match service.is_authenticated(&jar, &Role::Owner, "grant_token").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };

    match service.grant_token(body.uuid.as_str()).await {
        Ok(t) => (StatusCode::OK, format!("{{\"token\": \"{}\"}}", t)),
        Err(t) => (StatusCode::BAD_REQUEST, t.log_to_obfuscated(&user.uuid))
    }
}

