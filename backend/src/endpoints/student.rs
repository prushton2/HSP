use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::repository::student_repository;
use crate::service::student_service;
use crate::types::Role;


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
    let mut auth = state.auth.lock().await;
    if !auth.is_authenticated(&jar, &Role::Admin, "new_student").await { return (StatusCode::UNAUTHORIZED, String::from("")) }
    drop(auth);

    let mut service = state.student.lock().await;
    
    let student = student_repository::FullStudent {
        fname:    body.fname,
        lname:    body.lname,
        pronouns: body.pronouns,
        number:   body.number,
        hall:     body.hall,
        room:     body.room,
        wing:     body.wing,
    };

    let response = match service.create_student(&student).await {
        Ok(_) => (StatusCode::CREATED, "".to_string()),
        Err(t) => (StatusCode::BAD_REQUEST, String::from(t))
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
    let mut auth = state.auth.lock().await;
    if !auth.is_authenticated(&jar, &Role::Admin, "edit_student").await { return (StatusCode::UNAUTHORIZED, String::from("")) }
    drop(auth);

    let mut service = state.student.lock().await;
    
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
        "hall"       => {update.hall     = Some(body.str_field)},
        "wing"       => {update.room     = Some(body.int_field)},
        "room"       => {update.wing     = Some(body.str_field)},
        _            => { return (StatusCode::BAD_REQUEST, "Invalid Field".to_string())}
    }

    match service.update_student(&body.uuid, &update).await {
        Ok(_) => (StatusCode::OK, "".to_owned()),
        Err(t) => (StatusCode::INTERNAL_SERVER_ERROR, String::from(t))
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct GetStudent {
    pub uuid: String,
    pub decrypt: bool
}
pub async fn get_student(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<GetStudent>) -> (StatusCode, String) {
    let mut auth = state.auth.lock().await;
    if !auth.is_authenticated(&jar, &Role::Staff, "get_student").await { return (StatusCode::UNAUTHORIZED, String::from("")) }
    drop(auth);

    let mut service = state.student.lock().await;
    
    return match service.get_student(&body.uuid, body.decrypt).await {
        Ok(t) => (StatusCode::OK, serde_json::to_string(&t).unwrap()),
        Err(t) => (StatusCode::BAD_REQUEST, String::from(t))
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct DeleteStudent {
    pub uuid: String,
}
pub async fn delete_student(State(state): State<Arc<super::Services>>, jar: CookieJar, Json(body): Json<DeleteStudent>) -> (StatusCode, String) {
    let mut auth = state.auth.lock().await;
    if !auth.is_authenticated(&jar, &Role::Admin, "delete_student").await { return (StatusCode::UNAUTHORIZED, String::from("")) }
    drop(auth);

    let mut service = state.student.lock().await;
    
    let response = match service.delete_student(&body.uuid).await {
        Ok(_) => (StatusCode::OK, String::from("")),
        Err(t) => (StatusCode::BAD_REQUEST, String::from(t))
    };

    return response;
}