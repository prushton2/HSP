use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{repository::activities_repository::{SearchActivity, UpdateActivity}, types::{Error, Role}};

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

pub async fn edit_activity(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<UpdateActivity>) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Admin, "edit_activity").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    let service = state.activities.read().await;

    if body.uuid == "" {
        return (StatusCode::BAD_REQUEST, String::from("Provide a valid UUID"))
    }
    
    match service.edit_activity(body).await {
        Ok(_) => {},
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, t.log_to_obfuscated(&user.uuid))
    };

    (StatusCode::OK, "".to_owned())
}

#[derive(Serialize, Deserialize)]
pub struct GetActivityBody {
    uuid: String,
    get_attendees: bool,
    decrypt: bool
}
pub async fn get_activity(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<GetActivityBody>) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Staff, "get_activity_body").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    let service = state.activities.read().await;

    if body.uuid == "" {
        return (StatusCode::BAD_REQUEST, String::from("Provide a valid UUID"))
    }

    return match service.get_activity(&body.uuid, body.get_attendees, body.decrypt).await {
        Ok(t) => (StatusCode::OK, serde_json::to_string(&t).unwrap()),
        Err(t) => (StatusCode::INTERNAL_SERVER_ERROR, t.log_to_obfuscated(&user.uuid))
    }
}

#[derive(Serialize, Deserialize)]
pub struct BindActivityBody {
    uuid: String,
    student_numbers: Vec<i32>
}
pub async fn bind_activity(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<BindActivityBody>) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Admin, "bind_to_activity").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    if body.uuid == "" {
        return (StatusCode::BAD_REQUEST, String::from("Provide a valid UUID"))
    }

    if body.student_numbers.len() == 0 {
        return (StatusCode::BAD_REQUEST, String::from("Provide student IDs"))
    }

    let service = state.activities.read().await;

    match service.bind_students(&body.uuid, body.student_numbers).await {
        Ok(_) => {},
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, t.log_to_obfuscated(&user.uuid))
    }

    // bind_students(&self, uuid: &str, students: Vec<i32>);

    (StatusCode::OK, String::from(""))
}


#[derive(Serialize, Deserialize)]
pub struct SearchActivityBody {
    pub date: Option<i64>
}
pub async fn search_activity(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<SearchActivityBody>) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Staff, "get_activity_body").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    let service = state.activities.read().await;

    

    return match service.search_activity(&SearchActivity{name: None, staff: None, dates: body.date}).await {
        Ok(t) => (StatusCode::OK, serde_json::to_string(&t).unwrap()),
        Err(t) => (StatusCode::INTERNAL_SERVER_ERROR, t.log_to_obfuscated(&user.uuid))
    }
}