use axum::async_trait;

use crate::repository::StudentRepository;
use crate::repository::student_repository::{StudentEncrypted, UpdateStudentInfo, StudentResidence, UpdateStudentResidence, StudentInfo, SearchStudentInfo, UpdateStudentEncrypted, SearchStudentResidence};
use crate::types::Error;

#[async_trait]
impl StudentRepository for super::MockDB {
    async fn insert_studentinfo(&self, student: &StudentInfo) -> Result<(), Error> {
        let mut map = self.students.lock().unwrap();
        map.insert(student.uuid.clone(), StudentInfo {
            uuid:   student.uuid.clone(),
            number: student.number,
            fname:  student.fname.clone(),
            lname:  student.lname.clone(),
        });
        Ok(())
    }

    async fn update_studentinfo(&self, update: &UpdateStudentInfo) -> Result<(), Error> {
        let mut map = self.students.lock().unwrap();
        let entry = map.get_mut(&update.uuid)
            .ok_or(Error::InvalidParameter("uuid".to_owned(), update.uuid.clone()))?;

        if let Some(ref fname) = update.fname {
            entry.fname = fname.clone();
        }
        if let Some(ref lname) = update.lname {
            entry.lname = lname.clone();
        }
        if let Some(number) = update.number {
            entry.number = number;
        }
        Ok(())
    }

    async fn delete_studentinfo(&self, uuid: &str) -> Result<(), Error> {
        let mut map = self.students.lock().unwrap();
        map.remove(uuid);
        Ok(())
    }

    async fn get_studentinfo(&self, uuid: &str) -> Result<StudentInfo, Error> {
        let map = self.students.lock().unwrap();
        map.get(uuid)
            .map(|s| StudentInfo {
                uuid:   s.uuid.clone(),
                number: s.number,
                fname:  s.fname.clone(),
                lname:  s.lname.clone(),
            })
            .ok_or(Error::InvalidParameter("uuid".to_owned(), uuid.to_owned()))
    }

    async fn search_studentinfo(&self, params: &SearchStudentInfo) -> Result<Vec<StudentInfo>, Error> {
        let map = self.students.lock().unwrap();
        let results: Vec<StudentInfo> = map.values()
            .filter(|s| {
                if let Some(ref fname) = params.fname {
                    if s.fname != *fname { return false; }
                }
                if let Some(ref lname) = params.lname {
                    if s.lname != *lname { return false; }
                }
                if let Some(number) = params.number {
                    if s.number != number { return false; }
                }
                true
            })
            .map(|s| StudentInfo {
                uuid:   s.uuid.clone(),
                number: s.number,
                fname:  s.fname.clone(),
                lname:  s.lname.clone(),
            })
            .collect();
        Ok(results)
    }

    async fn insert_encrypted(&self, data: &StudentEncrypted) -> Result<(), Error> {
        let mut map = self.encrypted.lock().unwrap();
        map.insert(data.uuid.clone(), StudentEncrypted {
            uuid: data.uuid.clone(),
            data: data.data.clone(),
        });
        Ok(())
    }

    async fn update_encrypted(&self, update: &UpdateStudentEncrypted) -> Result<(), Error> {
        if update.data.is_none() {
            return Ok(());
        }
        let mut map = self.encrypted.lock().unwrap();
        let entry = map.get_mut(&update.uuid)
            .ok_or(Error::InvalidParameter("uuid".to_owned(), update.uuid.clone()))?;
        if let Some(ref data) = update.data {
            entry.data = data.clone();
        }
        Ok(())
    }

    async fn delete_encrypted(&self, uuid: &str) -> Result<(), Error> {
        let mut map = self.encrypted.lock().unwrap();
        map.remove(uuid);
        Ok(())
    }

    async fn get_encrypted(&self, uuid: &str) -> Result<StudentEncrypted, Error> {
        let map = self.encrypted.lock().unwrap();
        map.get(uuid)
            .map(|e| StudentEncrypted {
                uuid: e.uuid.clone(),
                data: e.data.clone(),
            })
            .ok_or(Error::InvalidParameter("uuid".to_owned(), uuid.to_owned()))
    }

    async fn getall_encrypted(&self) -> Result<Vec<StudentEncrypted>, Error> {
        let map = self.encrypted.lock().unwrap();
        Ok(map.values()
            .map(|e| StudentEncrypted {
                uuid: e.uuid.clone(),
                data: e.data.clone(),
            })
            .collect())
    }

    async fn insert_residence(&self, residence: &StudentResidence) -> Result<(), Error> {
        let mut map = self.residences.lock().unwrap();
        map.insert(residence.uuid.clone(), StudentResidence {
            uuid: residence.uuid.clone(),
            hall: residence.hall.clone(),
            room: residence.room,
            wing: residence.wing.clone(),
        });
        Ok(())
    }

    async fn update_residence(&self, update: &UpdateStudentResidence) -> Result<(), Error> {
        let mut map = self.residences.lock().unwrap();
        let entry = map.get_mut(&update.uuid)
            .ok_or(Error::InvalidParameter("uuid".to_owned(), update.uuid.clone()))?;

        if let Some(ref hall) = update.hall {
            entry.hall = hall.clone();
        }
        if let Some(room) = update.room {
            entry.room = room;
        }
        if let Some(ref wing) = update.wing {
            entry.wing = wing.clone();
        }
        Ok(())
    }

    async fn delete_residence(&self, uuid: &str) -> Result<(), Error> {
        let mut map = self.residences.lock().unwrap();
        map.remove(uuid);
        Ok(())
    }

    async fn get_residence(&self, uuid: &str) -> Result<StudentResidence, Error> {
        let map = self.residences.lock().unwrap();
        map.get(uuid)
            .map(|r| StudentResidence {
                uuid: r.uuid.clone(),
                hall: r.hall.clone(),
                room: r.room,
                wing: r.wing.clone(),
            })
            .ok_or(Error::InvalidParameter("uuid".to_owned(), uuid.to_owned()))
    }

    async fn search_residence(&self, params: &SearchStudentResidence) -> Result<Vec<StudentResidence>, Error> {
        let map = self.residences.lock().unwrap();
        let results: Vec<StudentResidence> = map.values()
            .filter(|r| {
                if let Some(ref hall) = params.hall {
                    if r.hall != *hall { return false; }
                }
                if let Some(room) = params.room {
                    if r.room != room { return false; }
                }
                if let Some(ref wing) = params.wing {
                    if r.wing != *wing { return false; }
                }
                true
            })
            .map(|r| StudentResidence {
                uuid: r.uuid.clone(),
                hall: r.hall.clone(),
                room: r.room,
                wing: r.wing.clone(),
            })
            .collect();
        Ok(results)
    }
}