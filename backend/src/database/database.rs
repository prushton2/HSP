use axum::async_trait;
use tokio_postgres::{error::SqlState, types::{ToSql, to_sql_checked}};
use uuid::Uuid;

use crate::{database::{self, Role, structs::UserInfo}, encryption::Encryption};

#[allow(dead_code)]


#[derive(Debug)]
pub enum Error {
    ErrorDuring(String, Box<Error>),
    InvalidParameter(String, String),
    PostgresError(Option<SqlState>),
    TokioError
}

#[async_trait]
pub trait Database: Send + Sync {
    fn get_encryption(&mut self) -> &mut dyn Encryption;

    async fn init_if_uninitialized(&mut self) -> Result<(), Error>;
    async fn get_student_tables(&mut self) -> Result<(
        Vec<database::TableStudentInfo>,
        Vec<database::TableResidencies>,
        Vec<database::TableStudentActivities>,
        Vec<database::TableActivities>
    ),
    Error>;

    async fn create_student(&mut self, user: &crate::endpoints::student::CreateUser) -> Result<(), Error>;
    async fn edit_student(&mut self, uuid: &str, field: &str, new_value: &FieldValue) -> Result<(), Error>;
    async fn get_student(&mut self, uuid: &str, decrypt: bool) -> Result<database::AllStudentInfo, Error>;
    async fn delete_student(&mut self, uuid: &str) -> Result<(), Error>;

    async fn create_user(&mut self, first_name: &str, last_name: &str, role: Role, device: &str) -> Result<Uuid, Error>;
    async fn grant_access(&mut self, uuid: &str, device: &str) -> Result<String, Error>;
    async fn get_user(&mut self, token: &str) -> Option<UserInfo>;
    async fn edit_user(&mut self, uuid: &str, field: &str, new_value: &FieldValue) -> Result<(), Error>;
    async fn delete_user(&mut self, uuid: &str) -> Result<(), Error>;
}

pub fn authenticate(method: &str, user: UserInfo, level: Role) -> bool {
    let authenticated = user.role >= level;



    authenticated
}

#[derive(Debug)]
pub enum FieldValue<'a> {
    Str(&'a str),
    I32(i32),
    Role(Role)
}

impl ToSql for FieldValue<'_> {
    fn to_sql(&self, _ty: &tokio_postgres::types::Type, out: &mut tokio_postgres::types::private::BytesMut) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + Sync + Send>>
        where
            Self: Sized {
        match self {
            FieldValue::Str(t) => t.to_sql(&tokio_postgres::types::Type::TEXT, out),
            FieldValue::I32(t) => t.to_sql(&tokio_postgres::types::Type::INT4, out),
            FieldValue::Role(t) =>  String::from(t).to_sql(&tokio_postgres::types::Type::TEXT, out),
        }
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool
        where
            Self: Sized {
        return ty == &tokio_postgres::types::Type::INT4 || ty == &tokio_postgres::types::Type::TEXT;
    }

    to_sql_checked!();
}