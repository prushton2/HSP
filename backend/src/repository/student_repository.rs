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

    async fn insert_encrypted(&self, data: &EncryptedInfo) -> Result<(), Error>;
    async fn update_encrypted(&self, update: &UpdateEncryptedInfo) -> Result<(), Error>;
    async fn delete_encrypted(&self, uuid: &str) -> Result<(), Error>;
    async fn get_encrypted   (&self, uuid: &str) -> Result<EncryptedInfo, Error>;
    async fn getall_encrypted(&self) -> Result<Vec<EncryptedInfo>, Error>;
    
    async fn insert_residence(&self, user: &ResidenceInfo) -> Result<(), Error>;
    async fn update_residence(&self, update: &UpdateResidenceInfo) -> Result<(), Error>;
    async fn delete_residence(&self, uuid: &str) -> Result<(), Error>;
    async fn get_residence   (&self, uuid: &str) -> Result<ResidenceInfo, Error>;
    async fn search_residence(&self, params: &SearchResidenceInfo) -> Result<Vec<ResidenceInfo>, Error>;
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
pub struct EncryptedInfo {
    pub uuid: String,
    pub data: String
}

pub struct UpdateEncryptedInfo {
    pub uuid: String,
    pub data: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct ResidenceInfo {
    pub uuid: String,
    pub hall: String,
    pub room: i32,
    pub wing: String
}

pub struct UpdateResidenceInfo {
    pub uuid: String,
    pub hall: Option<String>,
    pub room: Option<i32>,
    pub wing: Option<String>,
}

pub type SearchResidenceInfo = UpdateResidenceInfo;

