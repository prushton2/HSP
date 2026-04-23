use axum::async_trait;
use tokio_postgres::error::SqlState;


#[derive(Debug)]
pub enum Error {
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
}

#[derive(Debug)]
pub struct TableStudentInfo {
    pub uuid: String,
    pub number: u32,
}

pub struct TableResidencies {
    pub uuid: String,
    pub hall: String,
    pub room: u32,
    pub wing: String,
    pub role: String,
}

pub struct TableStudentActivities {
    pub uuid: String,
    pub date: u32,
    pub activity: String,
}

pub struct TableActivities {
    pub activity: String,
    pub date: u32,
    pub staff: [String; 8]
}