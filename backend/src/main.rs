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

    let db: Arc<Mutex<dyn database::Database>> = 
        Arc::new(
            Mutex::new(
                database::PSQLDB::new(&dbinfo).await
            )
        );

    {
        let mut reference = db.lock().await;
        let res = reference.init_if_uninitialized();
        println!("{:?}", res.await);
    }

    let app = Router::new()
        // .route("/admin/all", get(blah))
        .route("/student/new",   post(create_sudent))
        .route("/student/edit",  post(update_sudent))
        .route("/student/delete",post(delete_student))
        .route("/student/get",   post(get_student))

        .route("/auth/create", post(endpoints::auth::create_user))

        .with_state(db); // move db in directly, no clone needed

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();  
}
