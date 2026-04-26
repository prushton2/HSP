use serde::{Serialize, Deserialize};

pub mod psqldb;

pub use psqldb::PSQLDB;

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
    PostgresError(tokio_postgres::Error),
    TokioError,
    ExpiredError
}

impl From<Error> for String {
    fn from(value: Error) -> Self {
        match value {
            Error::ErrorDuring(m, e) => format!("{}: {}", m, String::from(*e)),
            Error::InvalidParameter(p, v) => format!("Invalid parameter {} supplied to {}", v, p),
            Error::PostgresError(e) => format!("Postgres Error: {}", fmt_pg_error(&e)),
            Error::TokioError => String::from("Tokio Error"),
            Error::ExpiredError => String::from("The resource you are trying to access expired"),
        }
    }
}

fn fmt_pg_error(e: &tokio_postgres::Error) -> String {
    if let Some(db) = e.as_db_error() {

        let mut msg = format!("{}: {}", db.severity(), db.message());

        if let Some(detail) = db.detail() {
            msg.push_str(&format!(" — {}", detail));
        }
        if let Some(hint) = db.hint() {
            msg.push_str(&format!(" (hint: {})", hint));
        }
        return msg

    }

    return e.to_string()
}