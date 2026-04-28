use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{Router, routing::{get, post}};
use uuid::Uuid;

use log::{LevelFilter};
use env_logger::Builder;

use crate::{repository::auth_repository::FullUser, types::Role};

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

    Builder::new()
        // Set project's max level
        .filter(Some("hsp_backend"), LevelFilter::Info)
        // turn off everything else
        .filter(None, LevelFilter::Off)
        .init();

    {
        let db = database::PSQLDB::new(&dbinfo).await;
        match db.init_if_uninitialized().await {
            Ok(_)         => log::info!("Database initialized"),
            Err(t) => log::error!("{}", t.to_obfuscated())
        };
    }

    let state = Arc::new(
        endpoints::Services { 
            student: RwLock::new(service::StudentService::new(
                Box::new(database::PSQLDB::new(&dbinfo).await),
                Box::new(encryption::PlaintextEncryption::new())
            )),
            admin: RwLock::new(service::AdminService::new(
                Box::new(database::PSQLDB::new(&dbinfo).await),
                Box::new(encryption::PlaintextEncryption::new())
            )),
            auth: RwLock::new(service::AuthService::new(
                Box::new(database::PSQLDB::new(&dbinfo).await),
                Box::new(encryption::PlaintextEncryption::new())
            )),
        }
    );

    match std::env::args().nth(1).as_deref() {
        Some("bootstrap-owner") => {
            let fname = std::env::args().nth(2).expect("usage: bootstrap-owner <fname> <lname>");
            let lname = std::env::args().nth(3).expect("usage: bootstrap-owner <fname> <lname>");
            let service = state.auth.write().await;
            let signup_hash = service.create_user(&FullUser {
                uuid: Uuid::new_v4().to_string(),
                fname: fname, 
                lname: lname,
                role: Role::Owner,
            }).await.expect("failed to create owner");
            println!("Signup link: /signup?token={}", signup_hash);
            return;
        }
        _ => {} // normal server startup
    }

    let app = Router::new()
        .route("/admin/all",      get(endpoints::admin::get_all_tables))

        .route("/student/new",   post(endpoints::student::new_sudent))
        .route("/student/edit",  post(endpoints::student::edit_student))
        .route("/student/delete",post(endpoints::student::delete_student))
        .route("/student/get",   post(endpoints::student::get_student))
        .route("/student/search",post(endpoints::student::search_students))

        .route("/auth/create", post(endpoints::auth::create_user))
        .route("/auth/signup", post(endpoints::auth::signup))
        .route("/auth/update", post(endpoints::auth::update_user))
        .route("/auth/delete", post(endpoints::auth::delete_user))
        
        .route("/auth/self",    get(endpoints::auth::get_self))

        .route("/auth/grant",  post(endpoints::auth::grant_token))
        .route("/auth/revoke", post(endpoints::auth::revoke_tokens))



        .with_state(state); // move db in directly, no clone needed

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();  
}


