use serde::{Serialize, Deserialize};

pub mod database;
pub mod psqldb;
pub mod structs;

pub use database::Database;
pub use database::Error;
pub use database::FieldValue;

pub use structs::AllStudentInfo;
pub use structs::TableStudentInfo;
pub use structs::TableResidencies;
pub use structs::TableStudentActivities;
pub use structs::TableActivities;


pub use psqldb::PSQLDB;

#[derive(Serialize, Deserialize, Clone)]
pub struct DBInfo {
    pub host: String,
    pub username: String,
    pub password: String,
    pub dbname: String,
    pub port: String
}