use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::{Router, routing::{get, post}};

mod database;
mod endpoints;
mod encryption;
mod repository;
mod service;
mod types;

const TOKEN_EXPIRY: i64 = 3024000;
const SIGNUP_HASH_EXPIRY: i64 = 86400;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // ignore error if .env doesn't exist (e.g. in prod)
    // now env::var() will see values from .env

    let dbinfo = database::DBInfo {
        username: std::env::var("POSTGRES_DB_USER").expect("Missing env var POSTGRES_DB_USER"),
        password: std::env::var("POSTGRES_DB_PASSWORD").expect("Missing env var POSTGRES_DB_PASSWORD"),
        dbname:   std::env::var("POSTGRES_DB_DATABASE").expect("Missing env var POSTGRES_DB_DATABASE"),
        host:     std::env::var("POSTGRES_DB_HOST").expect("Missing env var POSTGRES_DB_HOST"),
        port:     std::env::var("POSTGRES_DB_PORT").expect("Missing env var POSTGRES_DB_PORT")
    };

    {
        let mut db = database::PSQLDB::new(&dbinfo).await;
        match db.init_if_uninitialized().await {
            Ok(_)         => println!("Database initialized"),
            Err(t) => println!("{}", String::from(t))
        };
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
        .route("/admin/all",      get(endpoints::admin::get_all_tables))

        .route("/student/new",   post(endpoints::student::new_sudent))
        .route("/student/edit",  post(endpoints::student::edit_student))
        .route("/student/delete",post(endpoints::student::delete_student))
        .route("/student/get",   post(endpoints::student::get_student))

        .route("/auth/create", post(endpoints::auth::create_user))
        .route("/auth/signup", post(endpoints::auth::signup))
        .route("/auth/update", post(endpoints::auth::update_user))
        .route("/auth/delete", post(endpoints::auth::delete_user))

        .route("/auth/grant",  post(endpoints::auth::grant_token))
        .route("/auth/revoke", post(endpoints::auth::revoke_tokens))



        .with_state(state); // move db in directly, no clone needed

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();  
}


