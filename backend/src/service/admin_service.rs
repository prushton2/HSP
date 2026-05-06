// The service handles the actual logic to doing stuff to the database.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::encryption::Encryption;

use crate::repository::activities_repository::{Activity, SearchActivity};
use crate::types::Error;

use crate::repository::Repository;
use crate::repository::auth_repository::{User};
use crate::repository::student_repository::{StudentEncrypted, StudentResidence, SearchStudentResidence, SearchStudentInfo, StudentInfo};

pub struct AdminService {
    repo: Arc<dyn Repository>,
    _encryption: Arc<dyn Encryption>
}

#[allow(dead_code)]
impl AdminService {
    pub fn new(repo: Arc<dyn Repository>, encryption: Arc<dyn Encryption>) -> Self {
        Self {
            repo: repo,
            _encryption: encryption
        }
    }

    pub fn get_repo<'a>(&'a self) -> &'a dyn Repository {
        self.repo.as_ref()
    }

    pub async fn get_all_tables(&self) -> Result<AllTables, Error> {

        Ok(AllTables {
            residence: self.repo.search_residence(&SearchStudentResidence{uuid: String::from(""), hall: None, wing: None, room: None}).await?,
            studentinfo: self.repo.search_studentinfo(&SearchStudentInfo{uuid: String::from(""), fname: None, lname: None, number: None}).await?,
            encryptedinfo: self.repo.getall_encrypted().await?,
            users: self.repo.getall_user().await?,
            tokens: self.repo.getall_token().await?.iter().map(|f| {ObfuscatedTokenInfo {uuid: f.uuid.clone(), signed_up: f.signup_hash == "".to_owned(), expiry: f.expiry}}).collect(),
            activities: self.repo.search_activity(&SearchActivity{name: None, staff: None, dates: None}).await?
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct AllTables {
    pub residence: Vec<StudentResidence>,
    pub studentinfo: Vec<StudentInfo>,
    pub encryptedinfo: Vec<StudentEncrypted>,
    pub users: Vec<User>,
    pub tokens: Vec<ObfuscatedTokenInfo>,
    pub activities: Vec<Activity>
}

#[derive(Serialize, Deserialize)]
pub struct ObfuscatedTokenInfo {
    pub uuid: String,
    pub signed_up: bool,
    pub expiry: i64
}