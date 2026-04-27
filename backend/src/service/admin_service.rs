// The service handles the actual logic to doing stuff to the database.

use serde::{Deserialize, Serialize};

use crate::encryption::Encryption;

use crate::database::Error;

use crate::repository::Repository;
use crate::repository::auth_repository::{FullUser, TokenInfo};
use crate::repository::student_repository::{EncryptedInfo, ResidenceInfo, SearchResidenceInfo, SearchStudentInfo, StudentInfo};

pub struct AdminService {
    repo: Box<dyn Repository>,
    _encryption: Box<dyn Encryption>
}

impl AdminService {
    pub fn new(repo: Box<dyn Repository>, encryption: Box<dyn Encryption>) -> Self {
        Self {
            repo: repo,
            _encryption: encryption
        }
    }

    pub async fn get_all_tables(&mut self) -> Result<AllTables, Error> {

        Ok(AllTables {
            residence: self.repo.search_residence(&SearchResidenceInfo{uuid: String::from(""), hall: None, wing: None, room: None}).await?,
            studentinfo: self.repo.search_studentinfo(&SearchStudentInfo{uuid: String::from(""), fname: None, lname: None, number: None}).await?,
            encryptedinfo: self.repo.getall_encrypted().await?,
            users: self.repo.getall_user().await?,
            tokens: self.repo.getall_token().await?
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct AllTables {
    residence: Vec<ResidenceInfo>,
    studentinfo: Vec<StudentInfo>,
    encryptedinfo: Vec<EncryptedInfo>,
    users: Vec<FullUser>,
    tokens: Vec<TokenInfo>
}