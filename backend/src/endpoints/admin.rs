use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;

use crate::types::{Role, Error};


pub async fn get_all_tables(State(state): State<Arc<super::Services>>, jar: CookieJar) -> (StatusCode, String) {
    let user = match state.auth.is_authenticated(&jar, &Role::Admin, "get_all_tables").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };

    let mut tables = match state.admin.get_all_tables().await {
        Ok(t) => t,
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, t.log_to_obfuscated(&user.uuid))
    };

    if user.role == Role::Admin {
        tables.tokens = vec![];
        tables.users = vec![];
    }

    (StatusCode::OK, serde_json::to_string(&tables).unwrap())
}