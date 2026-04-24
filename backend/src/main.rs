use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::{Router, routing::{get, post}};

mod database;
mod endpoints;
mod encryption;

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
                database::PSQLDB::new(&dbinfo, Box::new(encryption::PlaintextEncryption::new())).await
            )
        );

    {
        let mut reference = db.lock().await;
        let res = reference.init_if_uninitialized();
        println!("{:?}", res.await);
    }

    let app = Router::new()
        .route("/student/all", get(endpoints::student::get_student_info))
        .route("/student/new", post(endpoints::student::new_sudent))
        .route("/student/edit", post(endpoints::student::edit_sudent))
        .route("/student/get", post(endpoints::student::get_student))
        .route("/student/delete", post(endpoints::student::delete_student))
        .with_state(db); // move db in directly, no clone needed

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();  
}
