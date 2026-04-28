use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;

use crate::types::{Role, Error};


pub async fn get_all_tables(State(state): State<Arc<super::Services>>, jar: CookieJar) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Admin, "get_all_tables").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    let service = state.admin.read().await;

    let tables = match service.get_all_tables().await {
        Ok(t) => t,
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, t.log_to_obfuscated(&user.uuid))
    };

    (StatusCode::OK, serde_json::to_string(&tables).unwrap())
}