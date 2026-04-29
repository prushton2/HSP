use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::service::student_service::{self, FullStudent, SearchStudent};
use crate::types::{Role, Error};


#[derive(Deserialize)]
pub struct CreateUser {
    pub fname: String,
    pub lname: String,
    pub pronouns: String,
    pub number: i32,
    pub hall: String,
    pub room: i32,
    pub wing: String,
}
pub async fn new_sudent(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<CreateUser>) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Admin, "new_student").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    let service = state.student.read().await;
    
    let student = FullStudent {
        fname:    body.fname,
        lname:    body.lname,
        pronouns: body.pronouns,
        number:   body.number,
        hall:     body.hall.to_ascii_lowercase(),
        room:     body.room,
        wing:     body.wing.to_ascii_lowercase(),
    };

    let response = match service.create_student(&student).await {
        Ok(_) => (StatusCode::CREATED, "".to_string()),
        Err(t) => (StatusCode::BAD_REQUEST, t.log_to_obfuscated(&user.uuid))
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
pub async fn edit_student(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<EditUser>) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Admin, "edit_student").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    let service = state.student.read().await;
    
    let mut update = student_service::StudentUpdate {
        fname:    None,
        lname:    None,
        pronouns: None,
        number:   None,
        hall:     None,
        room:     None,
        wing:     None
    };

    match body.field.as_str() {
        "first name" => {update.fname    = Some(body.str_field)},
        "last name"  => {update.lname    = Some(body.str_field)},
        "pronouns"   => {update.pronouns = Some(body.str_field)},
        "number"     => {update.number   = Some(body.int_field)},
        "hall"       => {update.hall     = Some(body.str_field.to_ascii_lowercase())},
        "wing"       => {update.wing     = Some(body.str_field.to_ascii_lowercase())},
        "room"       => {update.room     = Some(body.int_field)},
        _            => { return (StatusCode::BAD_REQUEST, "Invalid Field".to_string())}
    }

    match service.update_student(&body.uuid, &update).await {
        Ok(_) => (StatusCode::OK, "".to_owned()),
        Err(t) => (StatusCode::INTERNAL_SERVER_ERROR, t.log_to_obfuscated(&user.uuid))
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct GetStudent {
    pub uuid: String,
    pub decrypt: bool
}
pub async fn get_student(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<GetStudent>) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Staff, "get_student").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    let service = state.student.read().await;
    
    return match service.get_student(&body.uuid, body.decrypt).await {
        Ok(t) => (StatusCode::OK, serde_json::to_string(&t).unwrap()),
        Err(t) => (StatusCode::BAD_REQUEST, t.log_to_obfuscated(&user.uuid))
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DeleteStudent {
    pub uuid: String,
}
pub async fn delete_student(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<DeleteStudent>) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Admin, "delete_student").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    let service = state.student.read().await;
    
    let response = match service.delete_student(&body.uuid).await {
        Ok(_) => (StatusCode::OK, String::from("")),
        Err(t) => (StatusCode::BAD_REQUEST, t.log_to_obfuscated(&user.uuid))
    };

    return response;
}

pub async fn search_students(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<SearchStudent>) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Staff, "search_student").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    let service = state.student.read().await;
    
    return match service.search_students(&body).await {
        Ok(t) => (StatusCode::OK, serde_json::to_string(&t).unwrap()),
        Err(t) => (StatusCode::BAD_REQUEST, t.log_to_obfuscated(&user.uuid))
    };
}

#[derive(Deserialize)]
pub struct GetFromNumbers {
    numbers: Vec<i32>
}
pub async fn get_from_numbers(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<GetFromNumbers>) -> (StatusCode, String) {
    let auth = state.auth.read().await;
    let user = match auth.is_authenticated(&jar, &Role::Staff, "get_from_numbers").await {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Error::UnauthenticatedError.log_to_obfuscated("NO UUID"))
    };
    drop(auth);

    let service = state.student.read().await;

    let students = match service.get_from_numbers(body.numbers).await {
        Ok(t) => t,
        Err(t) => return (StatusCode::INTERNAL_SERVER_ERROR, t.log_to_obfuscated(&user.uuid))
    };

    (StatusCode::OK, serde_json::to_string(&students).unwrap())
}
