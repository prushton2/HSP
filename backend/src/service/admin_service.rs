// The service handles the actual logic to doing stuff to the database.

use serde::{Deserialize, Serialize};

use crate::encryption::Encryption;

use crate::database::{Database, Error};

use crate::repository::student_repository::{EncryptedInfo, ResidenceInfo, StudentInfo};

// #[derive(Clone)]
pub struct AdminService {
    repo: Box<dyn Database>,
    _encryption: Box<dyn Encryption>
}

impl AdminService {
    pub fn new(repo: Box<dyn Database>, encryption: Box<dyn Encryption>) -> Self {
        Self {
            repo: repo,
            _encryption: encryption
        }
    }


    pub async fn get_all_tables(&mut self) -> Result<AllTables, Error> {
        
        Ok(AllTables {
            residence: self.repo.getall_residence().await?,
            studentinfo: self.repo.getall_studentinfo().await?,
            encryptedinfo: self.repo.getall_encrypted().await?
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct AllTables {
    residence: Vec<ResidenceInfo>,
    studentinfo: Vec<StudentInfo>,
    encryptedinfo: Vec<EncryptedInfo>
}