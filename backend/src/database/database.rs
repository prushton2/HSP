use axum::async_trait;
use serde::{Deserialize, Serialize};
use tokio_postgres::error::SqlState;
#[allow(dead_code)]


#[derive(Debug)]
pub enum Error {
    ErrorDuring(String, Box<Error>),
    PostgresError(Option<SqlState>),
    TokioError
}

#[async_trait]
pub trait Database: Send + Sync {
    async fn init_if_uninitialized(&mut self) -> Result<(), Error>;
    async fn get_student_tables(&mut self) -> Result<(
        Vec<TableStudentInfo>,
        Vec<TableResidencies>,
        Vec<TableStudentActivities>,
        Vec<TableActivities>
    ),
    Error>;

    async fn create_student(&mut self, user: &crate::endpoints::admin::CreateUser) -> Result<(), Error>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableStudentInfo {
    pub uuid: String,
    pub number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableResidencies {
    pub uuid: String,
    pub hall: String,
    pub room: i32,
    pub wing: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableStudentActivities {
    pub uuid: String,
    pub date: chrono::NaiveDate,
    pub activity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableActivities {
    pub activity: String,
    pub date: chrono::NaiveDate,
    pub staff: Vec<String>
}