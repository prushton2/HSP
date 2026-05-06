use axum::async_trait;

use crate::repository::StudentRepository;
use crate::repository::student_repository::{StudentEncrypted, UpdateStudentInfo, StudentResidence, UpdateStudentResidence, StudentInfo, SearchStudentInfo, UpdateStudentEncrypted, SearchStudentResidence};
use crate::types::Error;

#[async_trait]
impl StudentRepository for super::MockDB {
    async fn insert_studentinfo(&self, student: &StudentInfo ) -> Result<(), Error> {
    }

    async fn update_studentinfo(&self, update: &UpdateStudentInfo) -> Result<(), Error> {
    }

    async fn delete_studentinfo(&self, uuid: &str) -> Result<(), Error> {
    }

    async fn get_studentinfo(&self, uuid: &str) -> Result<StudentInfo, Error> {
    }

    async fn search_studentinfo(&self, params: &SearchStudentInfo) -> Result<Vec<StudentInfo>, Error> {
    }

    async fn insert_encrypted(&self, data: &StudentEncrypted) -> Result<(), Error> {
    }

    async fn update_encrypted(&self, update: &UpdateStudentEncrypted) -> Result<(), Error> {
    }

    async fn delete_encrypted(&self, uuid: &str) -> Result<(), Error> {
    }

    async fn get_encrypted(&self, uuid: &str) -> Result<StudentEncrypted, Error> {
    }

    async fn getall_encrypted(&self) -> Result<Vec<StudentEncrypted>, Error> {
    }


    async fn insert_residence(&self, user: &StudentResidence) -> Result<(), Error> {
    }

    async fn update_residence(&self, update: &UpdateStudentResidence) -> Result<(), Error> {
    }

    async fn delete_residence(&self, uuid: &str) -> Result<(), Error> {
    }

    async fn get_residence(&self, uuid: &str) -> Result<StudentResidence, Error> {
    }

    async fn search_residence(&self, params: &SearchStudentResidence) -> Result<Vec<StudentResidence>, Error> {
    }
}