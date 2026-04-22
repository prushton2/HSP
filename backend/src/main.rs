use std::sync::{Arc, Mutex};
use axum::{extract::State, routing::get, Router};

mod database;

#[tokio::main]
async fn main() {
    let dbinfo = database::DBInfo {
        dbname: "db".to_string(),
        host: "localhost".to_string(),
        username: "user".to_string(),
        password: "password".to_string(),
        port: "5432".to_string()
    };

    let db: Arc<Mutex<Box<dyn database::Database + Send>>> = Arc::new(Mutex::new(Box::new(database::PSQLDB::new(&dbinfo))));

    let mut reference = db.lock().unwrap();
    let _ = reference.init_if_uninitialized();
    drop(reference);

    let app = Router::new()
        .route("/users", get(get_users))
        .with_state(Arc::clone(&db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Pool is injected automatically via State extractor
async fn get_users(State(pool): State<Arc<std::sync::Mutex<Box<dyn database::Database + Send>>>>) -> String {
// State(pool): State<PgPool>
    // let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
    //     .fetch_one(&pool)
    //     .await
    //     .unwrap();

    format!("{} users", 0)
}