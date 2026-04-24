use axum::async_trait;
use tokio_postgres::error::SqlState;

use crate::database;

#[allow(dead_code)]


#[derive(Debug)]
pub enum Error {
    ErrorDuring(String, Box<Error>),
    InvalidParameter(String, String),
    PostgresError(Option<SqlState>),
    TokioError
}

pub enum FieldValue<'a> {
    Str(&'a str),
    I32(i32)
}

#[async_trait]
pub trait Database: Send + Sync {
    async fn init_if_uninitialized(&mut self) -> Result<(), Error>;
    async fn get_student_tables(&mut self) -> Result<(
        Vec<database::TableStudentInfo>,
        Vec<database::TableResidencies>,
        Vec<database::TableStudentActivities>,
        Vec<database::TableActivities>
    ),
    Error>;

    async fn create_student(&mut self, user: &crate::endpoints::admin::CreateUser) -> Result<(), Error>;
    async fn edit_student(&mut self, uuid: &str, field: &str, new_value: &FieldValue) -> Result<(), Error>;
    async fn get_student(&mut self, uuid: &str, decrypt: bool) -> Result<database::AllStudentInfo, Error>;
}