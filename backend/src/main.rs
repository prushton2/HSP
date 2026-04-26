use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::{Router, routing::{get, post}};

mod database;
mod endpoints;
mod encryption;
mod repository;
mod service;
mod types;

#[tokio::main]
async fn main() {
    let dbinfo = database::DBInfo {
        dbname: "database".to_string(),
        host: "localhost".to_string(),
        username: "prushton".to_string(),
        password: "password".to_string(),
        port: "5432".to_string()
    };

    {
        let mut db = database::PSQLDB::new(&dbinfo).await;
        let res = db.init_if_uninitialized();
        println!("{:?}", res.await);
    }

    let state = Arc::new(
        endpoints::Services { 
            student: Mutex::new(service::StudentService::new(
                Box::new(database::PSQLDB::new(&dbinfo).await),
                Box::new(encryption::PlaintextEncryption::new())
            )),
            admin: Mutex::new(service::AdminService::new(
                Box::new(database::PSQLDB::new(&dbinfo).await),
                Box::new(encryption::PlaintextEncryption::new())
            )),
            auth: Mutex::new(service::AuthService::new(
                Box::new(database::PSQLDB::new(&dbinfo).await),
                Box::new(encryption::PlaintextEncryption::new())
            )),
        }
    );

    let app = Router::new()
        .route("/admin/all",      get(endpoints::admin::get_all_students))
        .route("/student/new",   post(endpoints::student::new_sudent))
        .route("/student/edit",  post(endpoints::student::edit_student))
        .route("/student/delete",post(endpoints::student::delete_student))
        .route("/student/get",   post(endpoints::student::get_student))

        .route("/auth/create", post(endpoints::auth::create_user))

        .with_state(state); // move db in directly, no clone needed

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();  
}


