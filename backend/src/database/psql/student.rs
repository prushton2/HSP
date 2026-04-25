use crate::repository::StudentRepository;
use crate::repository::student_repository::{CreateInfo, EncryptedInfo, FullStudent, InfoUpdate, ResidenceInfo, ResidenceUpdate, StudentInfo};

use crate::database::Error;

impl StudentRepository for super::PSQLDB {
    async fn insert_info(&mut self, uuid: &str, student: &CreateInfo) -> Result<(), Error> {
        return match self.client.execute("insert into studentinfo (UUID, number, first_name_hash, last_name_hash) values ($1, $2, $3, $4)", 
            &[&uuid, &student.number, &student.fname, &student.lname]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Inserting info".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };
    }
    
    async fn update_info(&mut self, uuid: &str, update: &InfoUpdate) -> Result<(), Error> {

    }
    async fn delete_info(&mut self, uuid: &str) -> Result<(), Error> {

    }
    async fn get_info   (&mut self, uuid: &str) -> Result<StudentInfo, Error> {

    }

    async fn insert_encrypted(&mut self, uuid: &str, data: &str) -> Result<(), Error> {

    }
    async fn update_encrypted(&mut self, uuid: &str, data: &str) -> Result<(), Error> {

    }
    async fn delete_encrypted(&mut self, uuid: &str) -> Result<(), Error> {

    }
    async fn get_encrypted   (&mut self, uuid: &str) -> Result<EncryptedInfo, Error> {

    }
    
    async fn insert_residence(&mut self, uuid: &str, user: &FullStudent) -> Result<(), Error> {

    }
    async fn update_residence(&mut self, uuid: &str, update: &ResidenceUpdate) -> Result<(), Error> {

    }
    async fn delete_residence(&mut self, uuid: &str) -> Result<(), Error> {

    }
    async fn get_residence   (&mut self, uuid: &str) -> Result<ResidenceInfo, Error> {

    }
}