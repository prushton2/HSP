use std::sync::{Arc};
use tokio::sync::Mutex;
use axum::{extract::State, routing::get, Router};

mod database;

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
        .route("/admin/get_student_info", get(get_student_info))
        .with_state(db); // move db in directly, no clone needed

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();  
}


async fn get_student_info(State(db_mutex): State<Arc<Mutex<dyn database::Database>>>) -> String {
    let mut db = db_mutex.lock().await;

    let all_tables = db.get_student_tables().await.unwrap();

    format!("{{\"student_info\":{},\"residencies\":{},\"student_activities\":{},\"activities\":{}}}",
        serde_json::to_string(&all_tables.0).unwrap(),
        serde_json::to_string(&all_tables.1).unwrap(),
        serde_json::to_string(&all_tables.2).unwrap(),
        serde_json::to_string(&all_tables.3).unwrap()
    )
}