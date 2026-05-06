// this holds the traits that directly interface with the database. These can be easily faked for tests.
use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::types::Error;

#[async_trait]
pub trait StudentRepository: Send + Sync {
    async fn insert_studentinfo(&self, student: &StudentInfo) -> Result<(), Error>;
    async fn update_studentinfo(&self, update: &UpdateStudentInfo) -> Result<(), Error>;
    async fn delete_studentinfo(&self, uuid: &str) -> Result<(), Error>;
    async fn get_studentinfo   (&self, uuid: &str) -> Result<StudentInfo, Error>;
    async fn search_studentinfo(&self, params: &SearchStudentInfo) -> Result<Vec<StudentInfo>, Error>;

    async fn insert_encrypted(&self, data: &StudentEncrypted) -> Result<(), Error>;
    async fn update_encrypted(&self, update: &UpdateStudentEncrypted) -> Result<(), Error>;
    async fn delete_encrypted(&self, uuid: &str) -> Result<(), Error>;
    async fn get_encrypted   (&self, uuid: &str) -> Result<StudentEncrypted, Error>;
    async fn getall_encrypted(&self) -> Result<Vec<StudentEncrypted>, Error>;
    
    async fn insert_residence(&self, user: &StudentResidence) -> Result<(), Error>;
    async fn update_residence(&self, update: &UpdateStudentResidence) -> Result<(), Error>;
    async fn delete_residence(&self, uuid: &str) -> Result<(), Error>;
    async fn get_residence   (&self, uuid: &str) -> Result<StudentResidence, Error>;
    async fn search_residence(&self, params: &SearchStudentResidence) -> Result<Vec<StudentResidence>, Error>;
}

#[derive(Serialize, Deserialize)]
pub struct StudentInfo {
    pub uuid:     String,
    pub number:   i32,
    pub fname:    String,
    pub lname:    String,
}

pub struct UpdateStudentInfo {
    pub uuid:   String,
    pub fname:  Option<String>,
    pub lname:  Option<String>,
    pub number: Option<i32>,
}

pub type SearchStudentInfo = UpdateStudentInfo;

#[derive(Serialize, Deserialize)]
pub struct StudentEncrypted {
    pub uuid: String,
    pub data: String
}

pub struct UpdateStudentEncrypted {
    pub uuid: String,
    pub data: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct StudentResidence {
    pub uuid: String,
    pub hall: String,
    pub room: i32,
    pub wing: String
}

pub struct UpdateStudentResidence {
    pub uuid: String,
    pub hall: Option<String>,
    pub room: Option<i32>,
    pub wing: Option<String>,
}

pub type SearchStudentResidence = UpdateStudentResidence;

