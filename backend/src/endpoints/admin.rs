use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;

use crate::types::Role;


pub async fn get_all_tables(State(state): State<Arc<super::Services>>, jar: CookieJar) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    if !auth.is_authenticated(&jar, &Role::Admin, "get_all_tables").await { return (StatusCode::UNAUTHORIZED, String::from("")) }
    drop(auth);


    let service = state.admin.read().await;

    let tables = match service.get_all_tables().await {
        Ok(t) => t,
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, String::from(t))
    };

    (StatusCode::OK, serde_json::to_string(&tables).unwrap())
}