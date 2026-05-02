use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::types::{Role, Error};

#[derive(Serialize, Deserialize)]
pub struct CreateActivity {
    pub name:  String,
    pub staff: Vec<String>,
    pub dates: Vec<i64>
}
pub async fn create_activity(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<CreateActivity>) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Admin, "create_activity").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    let service = state.activities.read().await;

    match service.create_activity(body).await {
        Ok(_) => {},
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, t.log_to_obfuscated(&user.uuid))
    };

    (StatusCode::OK, "".to_owned())
}