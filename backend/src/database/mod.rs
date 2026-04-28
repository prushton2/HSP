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