use serde::{Serialize, Deserialize};

pub mod psqldb;

pub use psqldb::PSQLDB;

use tokio_postgres::error::SqlState;

use crate::repository::StudentRepository;

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

pub trait Database: StudentRepository + Send + Sync {}