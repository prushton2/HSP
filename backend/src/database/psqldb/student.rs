use crate::repository::StudentRepository;
use crate::repository::student_repository::{CreateInfo, EncryptedInfo, FullStudent, InfoUpdate, ResidenceInfo, ResidenceUpdate, StudentInfo};

use crate::database::Error;

use axum::async_trait;
use tokio_postgres::types::ToSql;

#[async_trait]
impl StudentRepository for super::PSQLDB {
    async fn insert_studentinfo(&mut self, uuid: &str, student: &CreateInfo) -> Result<(), Error> {
        return match self.client.execute("insert into studentinfo (UUID, number, first_name_hash, last_name_hash) values ($1, $2, $3, $4)", 
            &[&uuid, &student.number, &student.fname, &student.lname]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Inserting info".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };
    }

    async fn update_studentinfo(&mut self, uuid: &str, update: &InfoUpdate) -> Result<(), Error> {
        // Build a dynamic UPDATE that only touches fields that are Some.
        let mut set_clauses: Vec<String> = Vec::new();
        let mut params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
        let mut idx = 1;

        if let Some(fname) = &update.fname {
            set_clauses.push(format!("first_name_hash = ${}", idx));
            params.push(Box::new(fname.clone()));
            idx += 1;
        }
        if let Some(lname) = &update.lname {
            set_clauses.push(format!("last_name_hash = ${}", idx));
            params.push(Box::new(lname.clone()));
            idx += 1;
        }
        if let Some(number) = &update.number {
            set_clauses.push(format!("number = ${}", idx));
            params.push(Box::new(*number));
            idx += 1;
        }

        if set_clauses.is_empty() {
            return Ok(());
        }

        params.push(Box::new(uuid.to_owned()));
        let query = format!("update studentinfo set {} where UUID = ${}", set_clauses.join(", "), idx);

        let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref() as &(dyn ToSql + Sync)).collect();

        return match self.client.execute(&query, &param_refs[..]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Updating info".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };
    }

    async fn delete_studentinfo(&mut self, uuid: &str) -> Result<(), Error> {
        return match self.client.execute("delete from studentinfo where UUID = $1", &[&uuid]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Deleting info".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };
    }

    async fn get_studentinfo(&mut self, uuid: &str) -> Result<StudentInfo, Error> {
        let row = match self.client.query_one("select number, first_name_hash, last_name_hash from studentinfo where UUID = $1", &[&uuid]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting info".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        Ok(StudentInfo {
            uuid:   uuid.to_string(),
            number: row.get::<&str, i32>("number"),
            fname:  row.get::<&str, &str>("first_name_hash").to_string(),
            lname:  row.get::<&str, &str>("last_name_hash").to_string(),
        })
    }

    async fn getall_studentinfo(&mut self) -> Result<Vec<StudentInfo>, Error> {
        let mut vec: Vec<StudentInfo> = vec![];
        
        let rows = match self.client.query("select * from studentinfo", &[]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting info".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        for row in rows {
            vec.push(StudentInfo { 
                uuid:   row.get("uuid"),
                number: row.get("number"),
                fname: row.get("first_name_hash"),
                lname: row.get("last_name_hash") 
            });
        }

        Ok(vec)
    }

    async fn insert_encrypted(&mut self, uuid: &str, data: &str) -> Result<(), Error> {
        return match self.client.execute("insert into encrypteddata (UUID, encrypted) values ($1, $2)", &[&uuid, &data]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Inserting encrypted".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };
    }

    async fn update_encrypted(&mut self, uuid: &str, data: &str) -> Result<(), Error> {
        return match self.client.execute("update encrypteddata set encrypted = $1 where UUID = $2", &[&data, &uuid]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Updating encrypted".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };
    }

    async fn delete_encrypted(&mut self, uuid: &str) -> Result<(), Error> {
        return match self.client.execute("delete from encrypteddata where UUID = $1", &[&uuid]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Deleting encrypted".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };
    }

    async fn get_encrypted(&mut self, uuid: &str) -> Result<EncryptedInfo, Error> {
        let row = match self.client.query_one("select encrypted from encrypteddata where UUID = $1", &[&uuid]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting encrypted".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        Ok(EncryptedInfo {
            uuid: uuid.to_string(),
            data: row.get::<&str, &str>("encrypted").to_string(),
        })
    }

    async fn getall_encrypted(&mut self) -> Result<Vec<EncryptedInfo>, Error> {
        let mut vec: Vec<EncryptedInfo> = vec![];

        let rows = match self.client.query("select * from encrypteddata", &[]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting encrypted".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        for row in rows {
            vec.push(EncryptedInfo{
                uuid: row.get("uuid"),
                data: row.get("encrypted")
            });
        }

        Ok(vec)
    }


    async fn insert_residence(&mut self, uuid: &str, user: &FullStudent) -> Result<(), Error> {
        return match self.client.execute("insert into residencies (UUID, hall, room, wing) values ($1, $2, $3, $4)",
            &[&uuid, &user.hall, &user.room, &user.wing]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Inserting residence".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };
    }

    async fn update_residence(&mut self, uuid: &str, update: &ResidenceUpdate) -> Result<(), Error> {
        let mut set_clauses: Vec<String> = Vec::new();
        let mut params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
        let mut idx = 1;

        if let Some(hall) = &update.hall {
            set_clauses.push(format!("hall = ${}", idx));
            params.push(Box::new(hall.clone()));
            idx += 1;
        }
        if let Some(room) = &update.room {
            set_clauses.push(format!("room = ${}", idx));
            params.push(Box::new(*room));
            idx += 1;
        }
        if let Some(wing) = &update.wing {
            set_clauses.push(format!("wing = ${}", idx));
            params.push(Box::new(wing.clone()));
            idx += 1;
        }

        if set_clauses.is_empty() {
            return Ok(());
        }

        params.push(Box::new(uuid.to_owned()));
        let query = format!("update residencies set {} where UUID = ${}", set_clauses.join(", "), idx);

        let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref() as &(dyn ToSql + Sync)).collect();

        return match self.client.execute(&query, &param_refs[..]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Updating residence".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };
    }

    async fn delete_residence(&mut self, uuid: &str) -> Result<(), Error> {
        return match self.client.execute("delete from residencies where UUID = $1", &[&uuid]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Deleting residence".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };
    }

    async fn get_residence(&mut self, uuid: &str) -> Result<ResidenceInfo, Error> {
        let row = match self.client.query_one("select * from residencies where UUID = $1", &[&uuid]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting residence".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        Ok(ResidenceInfo {
            uuid: uuid.to_string(),
            hall: row.get::<&str, &str>("hall").to_string(),
            room: row.get::<&str, i32>("room"),
            wing: row.get::<&str, &str>("wing").to_string(),
        })
    }

    async fn getall_residence(&mut self) -> Result<Vec<ResidenceInfo>, Error> {
        let mut vec: Vec<ResidenceInfo> = vec![];

        let rows = match self.client.query("select * from residencies", &[]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting residence".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        for row in rows {
            vec.push(ResidenceInfo{
                uuid: row.get("uuid"),
                hall: row.get::<&str, &str>("hall").to_string(),
                room: row.get::<&str, i32>("room"),
                wing: row.get::<&str, &str>("wing").to_string(),
            })
        }

        Ok(vec)
    }
}