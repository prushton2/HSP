// The service handles the actual logic to doing stuff to the database.

use uuid::Uuid;

use crate::repository::StudentRepository;
use crate::repository::student_repository::{self, CreateInfo, InfoUpdate};

use crate::encryption::{Encryption, EncryptedContents};

use crate::database::Error;

use crate::types::Role;

pub struct StudentService<R: StudentRepository> {
    repo: R,
    encryption: Box<dyn Encryption>
}

impl<R: StudentRepository> StudentService<R> {
    pub fn new(repo: R, encryption: Box<dyn Encryption>) -> Self {
        Self {
            repo: repo,
            encryption: encryption
        }
    }

    pub async fn create_student(&mut self, student: &student_repository::CreateStudent) -> Result<(), Error> {
        let uuid = Uuid::new_v4().to_string();

        match self.repo.insert_residence(&uuid, student).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting Residence".to_owned(), Box::new(t)))
        };

        let encrypted = self.encryption.encrypt(&EncryptedContents {
            first_name: student.fname.clone(),
            last_name: student.lname.clone(),
            pronouns: student.pronouns.clone()
        });

        match self.repo.insert_encrypted(&uuid, &encrypted).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Insert Encrypted".to_owned(), Box::new(t)))
        };

        match self.repo.insert_info(&uuid, &CreateInfo {
            fname: self.encryption.hash(&student.fname, ""),
            lname: self.encryption.hash(&student.lname, ""),
            number: student.number
        }).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting info".to_owned(), Box::new(t)))
        }

        Ok(())
    }

    pub async fn update_student(&mut self, uuid: &str, update: &StudentUpdate) -> Result<(), Error> {
        if update.fname.is_some() || update.lname.is_some() || update.pronouns.is_some() {
            // Update these encrypted values
            let mut current_info = match self.repo.get_encrypted(uuid).await {
                Ok(t) => self.encryption.decrypt(&t.data),
                Err(t) => return Err(Error::ErrorDuring("Decrypting student".to_owned(), Box::new(t)))
            };

            current_info.first_name = if update.fname.is_some() { update.fname.clone().unwrap() } else { current_info.first_name };
            current_info.last_name = if update.lname.is_some() { update.lname.clone().unwrap() } else { current_info.last_name };
            current_info.pronouns = if update.pronouns.is_some() { update.pronouns.clone().unwrap() } else { current_info.pronouns };

            let encrypted = self.encryption.encrypt(&current_info);

            match self.repo.update_encrypted(uuid, &encrypted).await {
                Ok(_) => {},
                Err(t) => return Err(Error::ErrorDuring("Updaing encrypted data".to_owned(), Box::new(t)))
            };
        }

        if update.number.is_some() || update.fname.is_some() || update.lname.is_some() {
            let new_info = InfoUpdate {
                number: if update.number.is_some() { update.number } else { None },
                fname: if update.fname.is_some() { update.fname.clone() } else { None },
                lname: if update.lname.is_some() { update.lname.clone() } else { None }
            };

            match self.repo.update_info(uuid, &new_info).await {
                Ok(_) => {},
                Err(t) => return Err(Error::ErrorDuring("Updating info".to_owned(), Box::new(t)))
            }
        }
        Ok(())
    }

    pub async fn delete_student(&mut self, uuid: &str) -> Result<(), Error> {
        Ok(())
    }

    pub async fn get_student(&mut self, uuid: &str, decrypt: bool) -> Result<(), Error> {
        Ok(())
    }
}

pub struct StudentUpdate {
    pub fname:    Option<String>,
    pub lname:    Option<String>,
    pub pronouns: Option<String>,
    pub number:   Option<i32>,
    pub hall:     Option<String>,
    pub room:     Option<i32>,
    pub wing:     Option<String>,
    pub role:     Option<Role>,
}