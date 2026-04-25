use axum::async_trait;

// this holds the traits that directly interface with the database. These can be easily faked for tests.
use crate::database::Error;
use crate::types::Role;

#[async_trait]
pub trait StudentRepository: Send + Sync {
    async fn insert_studentinfo(&mut self, uuid: &str, student: &CreateInfo) -> Result<(), Error>;
    async fn update_studentinfo(&mut self, uuid: &str, update: &InfoUpdate) -> Result<(), Error>;
    async fn delete_studentinfo(&mut self, uuid: &str) -> Result<(), Error>;
    async fn get_studentinfo   (&mut self, uuid: &str) -> Result<StudentInfo, Error>;
    async fn getall_studentinfo(&mut self) -> Result<Vec<StudentInfo>, Error>;

    async fn insert_encrypted(&mut self, uuid: &str, data: &str) -> Result<(), Error>;
    async fn update_encrypted(&mut self, uuid: &str, data: &str) -> Result<(), Error>;
    async fn delete_encrypted(&mut self, uuid: &str) -> Result<(), Error>;
    async fn get_encrypted   (&mut self, uuid: &str) -> Result<EncryptedInfo, Error>;
    async fn getall_encrypted(&mut self) -> Result<Vec<EncryptedInfo>, Error>;
    
    async fn insert_residence(&mut self, uuid: &str, user: &FullStudent) -> Result<(), Error>;
    async fn update_residence(&mut self, uuid: &str, update: &ResidenceUpdate) -> Result<(), Error>;
    async fn delete_residence(&mut self, uuid: &str) -> Result<(), Error>;
    async fn get_residence   (&mut self, uuid: &str) -> Result<ResidenceInfo, Error>;
    async fn getall_residence(&mut self) -> Result<Vec<ResidenceInfo>, Error>;
}

pub struct StudentInfo {
    pub uuid: String,
    pub number: i32,
    pub fname: String,
    pub lname: String,
}

pub struct EncryptedInfo {
    pub uuid: String,
    pub data: String
}

pub struct ResidenceInfo {
    pub uuid: String,
    pub hall: String,
    pub room: i32,
    pub wing: String,
    pub role: Role,
}

pub struct ResidenceUpdate {
    pub hall: Option<String>,
    pub room: Option<i32>,
    pub wing: Option<String>,
    pub role: Option<Role>,
}

pub struct InfoUpdate {
    pub fname:  Option<String>,
    pub lname:  Option<String>,
    pub number: Option<i32>,
}

pub struct CreateInfo {
    pub fname:  String,
    pub lname:  String,
    pub number: i32,
}

pub struct FullStudent {
    pub fname: String,
    pub lname: String,
    pub pronouns: String,
    pub number: i32,
    pub hall: String,
    pub room: i32,
    pub wing: String,
    pub role: Role,
}

impl Default for FullStudent {
    fn default() -> Self {
        Self {
            fname:    String::new(),
            lname:    String::new(),
            pronouns: String::new(),
            hall:     String::new(),
            wing:     String::new(),
            number:   0,
            room:     0,
            role:     Role::Staff,
        }
    }
}