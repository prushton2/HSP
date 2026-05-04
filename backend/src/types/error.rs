use tokio_postgres::error::SqlState;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    ErrorDuring(String, Box<Error>),
    InvalidParameter(String, String), // parameter, value
    PostgresError(tokio_postgres::Error),
    TokioError,
    ExpiredError,
    UnauthenticatedError
}

impl Error {
    pub fn to_obfuscated(&self) -> String {
        match self {
            Error::ErrorDuring(_, e) => (*e).to_obfuscated(),
            Error::InvalidParameter(p, v) => format!("Invalid parameter {} supplied to {}", v, p),
            Error::PostgresError(t) => Self::obfuscate_psql_err(t),
            Error::TokioError => String::from("Tokio error"),
            Error::ExpiredError => String::from("The resource you are trying to access expired"),
            Error::UnauthenticatedError => String::from("Unauthenticated"),
        }
    }

    fn obfuscate_psql_err(err: &tokio_postgres::Error) -> String {
        match err.code() {
            Some(&SqlState::UNIQUE_VIOLATION) => format!("{}", err.as_db_error().unwrap().detail().unwrap()),
            _ => String::from("Database Error")
        }
    }

    pub fn to_deobfuscated(&self) -> String {
        match self {
            Error::ErrorDuring(m, e) => format!("{}: {}", m, (*e).to_deobfuscated()),
            Error::InvalidParameter(p, v) => format!("Invalid parameter {} supplied to {}", v, p),
            Error::PostgresError(e) => format!("Postgres Error: {}", fmt_pg_error(&e)),
            Error::TokioError => String::from("Tokio Error"),
            Error::ExpiredError => String::from("The resource you are trying to access expired"),
            Error::UnauthenticatedError => String::from("Unauthenticated"),
        }
    }

    pub fn log_custom(uuid: &str, reason: &str) {
        log::error!("[{}]: {}", uuid, reason);
    }

    pub fn log(&self, uuid: &str) {
        log::error!("[{}]: {}", uuid, self.to_deobfuscated());
    }

    pub fn log_to_obfuscated(&self, uuid: &str) -> String {
        self.log(uuid);
        return self.to_obfuscated();
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