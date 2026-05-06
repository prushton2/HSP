// The service handles the actual logic to doing stuff to the database.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repository::Repository;
use crate::repository::student_repository::{StudentEncrypted, StudentResidence, SearchStudentResidence, SearchStudentInfo, StudentInfo, UpdateStudentEncrypted, UpdateStudentResidence, UpdateStudentInfo};

use crate::encryption::{Encryption, EncryptedContents};

use crate::types::Error;

pub struct StudentService {
    repo: Box<dyn Repository>,
    encryption: Arc<dyn Encryption>
}

impl StudentService {
    pub fn new(repo: Box<dyn Repository>, encryption: Arc<dyn Encryption>) -> Self {
        Self {
            repo: repo,
            encryption: encryption
        }
    }

    pub async fn create_student(&self, student: FullStudent) -> Result<(), Error> {
        let uuid = Uuid::new_v4().to_string();

        let student_info = StudentInfo {
            uuid: uuid.clone(),
            fname: self.encryption.hash(&student.fname.to_ascii_lowercase(), ""),
            lname: self.encryption.hash(&student.lname.to_ascii_lowercase(), ""),
            number: student.number
        };

        match self.repo.insert_studentinfo(&student_info).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting info".to_owned(), Box::new(t)))
        }

        let residence = StudentResidence {
            uuid: uuid.clone(),
            hall: student.hall,
            room: student.room,
            wing: student.wing
        };

        match self.repo.insert_residence(&residence).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting Residence".to_owned(), Box::new(t)))
        };

        let encrypted = self.encryption.encrypt(&EncryptedContents {
            first_name: student.fname,
            last_name: student.lname,
            pronouns: student.pronouns
        });

        let encrypted_info = StudentEncrypted {
            uuid: uuid,
            data: encrypted
        };

        match self.repo.insert_encrypted(&encrypted_info).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Insert Encrypted".to_owned(), Box::new(t)))
        };

        Ok(())
    }

    pub async fn update_student(&self, uuid: &str, update: &StudentUpdate) -> Result<(), Error> {
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

            let new_encrypted = UpdateStudentEncrypted {
                uuid: uuid.to_string(),
                data: Some(encrypted)
            };

            match self.repo.update_encrypted(&new_encrypted).await {
                Ok(_) => {},
                Err(t) => return Err(Error::ErrorDuring("Updaing encrypted data".to_owned(), Box::new(t)))
            };
        }

        if update.number.is_some() || update.fname.is_some() || update.lname.is_some() {
            let new_info = UpdateStudentInfo {
                uuid: uuid.to_string(),
                number: if update.number.is_some() { update.number } else { None },
                fname: if update.fname.is_some() { Some(update.fname.as_ref().unwrap().to_ascii_lowercase()) } else { None },
                lname: if update.lname.is_some() { Some(update.lname.as_ref().unwrap().to_ascii_lowercase()) } else { None }
            };

            match self.repo.update_studentinfo(&new_info).await {
                Ok(_) => {},
                Err(t) => return Err(Error::ErrorDuring("Updating info".to_owned(), Box::new(t)))
            }
        }

        if update.room.is_some() || update.wing.is_some() || update.hall.is_some() {
            let new_info = UpdateStudentResidence {
                uuid: uuid.to_string(),
                room: update.room.clone(),
                wing: update.wing.clone(),
                hall: update.hall.clone()
            };

            match self.repo.update_residence(&new_info).await {
                Ok(_) => {},
                Err(t) => return Err(Error::ErrorDuring("Updating Residence".to_owned(), Box::new(t)))
            }
        }
        Ok(())
    }

    pub async fn delete_student(&self, uuid: &str) -> Result<(), Error> {
        match self.repo.delete_encrypted(uuid).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Deleting Encrypted".to_owned(), Box::new(t)))
        }

        match self.repo.delete_residence(uuid).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Deleting Residence".to_owned(), Box::new(t)))
        }

        match self.repo.delete_studentinfo(uuid).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Deleting Info".to_owned(), Box::new(t)))
        }

        Ok(())
    }

    pub async fn get_student(&self, uuid: &str, decrypt: bool) -> Result<FullStudent, Error> {
        let mut student = FullStudent::default();

        if decrypt {
            let info = match self.repo.get_encrypted(uuid).await {
                Ok(t) => self.encryption.decrypt(&t.data),
                Err(t) => return Err(Error::ErrorDuring("Getting encrypted data".to_owned(), Box::new(t)))
            };

            student.fname = info.first_name;
            student.lname = info.last_name;
            student.pronouns = info.pronouns;
        }

        let info = match self.repo.get_studentinfo(uuid).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting info".to_owned(), Box::new(t)))
        };

        student.number = info.number;

        let info = match self.repo.get_residence(uuid).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting residence".to_owned(), Box::new(t)))
        };

        student.hall = info.hall;
        student.room = info.room;
        student.wing = info.wing;

        Ok(student)
    }

    // TODO: FIX
    pub async fn search_students(&self, params: &SearchStudent) -> Result<Vec<FullStudent>, Error> {
        let mut info_uuids: HashSet<String> = [].into();
        let mut res_uuids:  HashSet<String> = [].into();

        let student_info_params = SearchStudentInfo {
            uuid: String::from(""),
            fname: params.fname.clone(),
            lname: params.lname.clone(),
            number: params.number,
        };

        let residence_info_params = SearchStudentResidence {
            uuid: String::from(""),
            hall: params.hall.clone(),
            room: params.room,
            wing: None
        };

        // fetch UUIDs of students that match the parameters
        match self.repo.search_studentinfo(&student_info_params).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Searching student info".to_owned(), Box::new(t)))
        }.iter().for_each(|f| {info_uuids.insert(f.uuid.clone());});

        match self.repo.search_residence(&residence_info_params).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Searching residence info".to_owned(), Box::new(t)))
        }.iter().for_each(|f| {res_uuids.insert(f.uuid.clone());});

        let uuids = info_uuids.into_iter().filter(|f| res_uuids.contains(f)).collect::<Vec<String>>();

        let mut student_info: Vec<FullStudent> = vec![];

        // fetch the student info
        for uuid in uuids {
            let mut student = FullStudent::default();

            let info = match self.repo.get_encrypted(&uuid).await {
                Ok(t) => t,
                Err(t) => return Err(Error::ErrorDuring("Searching residence info".to_owned(), Box::new(t)))
            };

            let decrypted = self.encryption.decrypt(&info.data);

            student.fname = decrypted.first_name;
            student.lname = decrypted.last_name;
            student.pronouns = decrypted.pronouns;

            let info = match self.repo.get_studentinfo(&uuid).await {
                Ok(t) => t,
                Err(t) => return Err(Error::ErrorDuring("Searching residence info".to_owned(), Box::new(t)))
            };

            student.number = info.number;

            let info = match self.repo.get_residence(&uuid).await {
                Ok(t) => t,
                Err(t) => return Err(Error::ErrorDuring("Searching residence info".to_owned(), Box::new(t)))
            };

            student.hall = info.hall;
            student.wing = info.wing;
            student.room = info.room;

            student_info.push(student);
        }

        Ok(student_info)
    }

    pub async fn get_from_numbers(&self, numbers: Vec<i32>) -> Result<Vec<FullStudent>, Error> {
        let hashset_numbers: HashSet<i32> = HashSet::from_iter(numbers);
        let mut map: HashMap<&String, FullStudent> = [].into();

        let numbers = match self.repo.search_studentinfo(&UpdateStudentInfo { uuid: "".to_owned(), fname: None, lname: None, number: None }).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting student info".to_owned(), Box::new(t)))
        };

        numbers.iter().for_each(|e| {
            if hashset_numbers.contains(&e.number) {
                map.insert(
                    &e.uuid,
                    FullStudent { 
                        fname: String::from(""),
                        lname: String::from(""),
                        pronouns: String::from(""),
                        number: e.number,
                        hall: String::from(""),
                        room: -1,
                        wing: String::from("")
                    }
                );
            }
        });

        let residences = match self.repo.search_residence(&SearchStudentResidence {uuid: String::from(""), hall: None, wing: None, room: None}).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting residence info".to_owned(), Box::new(t)))
        };

        residences.iter().for_each(|e| {
            if let Some(reference) = map.get_mut(&e.uuid) {
                reference.hall = e.hall.clone();
                reference.room = e.room;
                reference.wing = e.wing.clone();
            }
        });

        let encrypted = match self.repo.getall_encrypted().await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting residence info".to_owned(), Box::new(t)))
        };

        encrypted.iter().for_each(|e| {
            if let Some(reference) = map.get_mut(&e.uuid) {
                let decrypted = self.encryption.decrypt(&e.data);
                reference.fname = decrypted.first_name;
                reference.lname = decrypted.last_name;
                reference.pronouns = decrypted.pronouns;
            }
        });



        return Ok(map.into_iter().map(|(_, v)| {v}).collect::<Vec<FullStudent>>());
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
}

#[derive(Serialize, Deserialize)]
pub struct SearchStudent {
    pub fname:    Option<String>,
    pub lname:    Option<String>,
    pub number:   Option<i32>,
    pub hall:     Option<String>,
    pub room:     Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct FullStudent {
    pub fname: String,
    pub lname: String,
    pub pronouns: String,
    pub number: i32,
    pub hall: String,
    pub room: i32,
    pub wing: String,
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
        }
    }
}