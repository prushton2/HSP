use serde::{Serialize, Deserialize};

pub mod psql;

use tokio_postgres::error::SqlState;

#[derive(Serialize, Deserialize, Clone)]
pub struct DBInfo {
    pub host: String,
    pub username: String,
    pub password: String,
    pub dbname: String,
    pub port: String
}

#[derive(Debug)]
pub enum Error {
    ErrorDuring(String, Box<Error>),
    InvalidParameter(String, String),
    PostgresError(Option<SqlState>),
    TokioError
}