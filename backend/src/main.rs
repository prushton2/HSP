use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::{Router, routing::{get, post}};

mod database;
mod endpoints;

#[tokio::main]
async fn main() {
    let dbinfo = database::DBInfo {
        dbname: "database".to_string(),
        host: "localhost".to_string(),
        username: "prushton".to_string(),
        password: "password".to_string(),
        port: "5432".to_string()
    };
    let db: Arc<Mutex<dyn database::Database>> = Arc::new(Mutex::new(database::PSQLDB::new(&dbinfo).await));

    {
        let mut reference = db.lock().await;
        let res = reference.init_if_uninitialized();
        println!("{:?}", res.await);
    }

    let app = Router::new()
        .route("/admin/get_student_info", get(endpoints::admin::get_student_info))
        .route("/admin/student/new", post(endpoints::admin::new_sudent))
        .with_state(db); // move db in directly, no clone needed

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();  
}


