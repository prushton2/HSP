use serde::{Serialize, Deserialize};

pub mod database;
pub mod psqldb;

pub use database::Database;
pub use database::Error;
pub use database::FieldValue;

pub use database::TableStudentInfo;
pub use database::TableResidencies;
pub use database::TableStudentActivities;
pub use database::TableActivities;


pub use psqldb::PSQLDB;

#[derive(Serialize, Deserialize, Clone)]
pub struct DBInfo {
    pub host: String,
    pub username: String,
    pub password: String,
    pub dbname: String,
    pub port: String
}