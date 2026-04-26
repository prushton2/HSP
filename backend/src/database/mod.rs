use serde::{Serialize, Deserialize};

pub mod psqldb;

pub use psqldb::PSQLDB;

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
    InvalidParameter(String, String), // parameter, value
    PostgresError(Option<SqlState>),
    TokioError
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        match value {
            Error::ErrorDuring(m, e) => format!("{}: {}", m, String::from(*e)),
            Error::InvalidParameter(p, v) => format!("Invalid parameter {} supplied to {}", v, p),
            Error::PostgresError(e) if e.is_some() => format!("Postgres Error code {}", e.unwrap().code()),
            Error::PostgresError(e) if e.is_none() => format!("Generic Posgres Error"),
            Error::TokioError => String::from("Tokio Error"),
            _ => format!("Invalid Error (this shouldnt happen!)")
        }
    }
}