use crate::repository::StudentRepository;
use crate::repository::student_repository::{StudentEncrypted, UpdateStudentInfo, StudentResidence, UpdateStudentResidence, StudentInfo, SearchStudentInfo, UpdateStudentEncrypted, SearchStudentResidence};

use crate::types::Error;

use axum::async_trait;
use tokio_postgres::types::ToSql;

#[async_trait]
impl StudentRepository for super::PSQLDB {
    async fn insert_studentinfo(&self, student: &StudentInfo ) -> Result<(), Error> {
        return match self.client.execute("insert into studentinfo (UUID, number, first_name_hash, last_name_hash) values ($1, $2, $3, $4)", 
            &[&student.uuid, &student.number, &student.fname, &student.lname]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Inserting info".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn update_studentinfo(&self, update: &UpdateStudentInfo) -> Result<(), Error> {
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

        params.push(Box::new(update.uuid.to_owned()));
        let query = format!("update studentinfo set {} where UUID = ${}", set_clauses.join(", "), idx);

        let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref() as &(dyn ToSql + Sync)).collect();

        return match self.client.execute(&query, &param_refs[..]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Updating info".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn delete_studentinfo(&self, uuid: &str) -> Result<(), Error> {
        return match self.client.execute("delete from studentinfo where UUID = $1", &[&uuid]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Deleting info".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn get_studentinfo(&self, uuid: &str) -> Result<StudentInfo, Error> {
        let row = match self.client.query_one("select number, first_name_hash, last_name_hash from studentinfo where UUID = $1", &[&uuid]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting info".to_owned(), Box::new(Error::PostgresError(t))))
        };

        Ok(StudentInfo {
            uuid:   uuid.to_string(),
            number: row.get::<&str, i32>("number"),
            fname:  row.get::<&str, &str>("first_name_hash").to_string(),
            lname:  row.get::<&str, &str>("last_name_hash").to_string(),
        })
    }

    async fn search_studentinfo(&self, params: &SearchStudentInfo) -> Result<Vec<StudentInfo>, Error> {
        // Dynamically build query
        let mut clauses: Vec<String> = Vec::new();
        let mut query_params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
        let mut idx = 1;

        if let Some(fname) = &params.fname {
            clauses.push(format!("first_name_hash = ${}", idx));
            query_params.push(Box::new(fname.clone()));
            idx += 1;
        }
        if let Some(lname) = &params.lname {
            clauses.push(format!("last_name_hash = ${}", idx));
            query_params.push(Box::new(lname.clone()));
            idx += 1;
        }
        if let Some(number) = &params.number {
            clauses.push(format!("number = ${}", idx));
            query_params.push(Box::new(*number));
        }

        let param_refs: Vec<&(dyn ToSql + Sync)> = query_params.iter().map(|p| p.as_ref() as &(dyn ToSql + Sync)).collect();
        
        let query_string = format!("select * from studentinfo {} {}",
            if clauses.len() != 0 { "where" } else {""},
            clauses.join(" and ")
        );
        
        let rows = match self.client.query(&query_string, &param_refs[..]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting info".to_owned(), Box::new(Error::PostgresError(t))))
        };

        let mut vec: Vec<StudentInfo> = vec![];
        for row in rows {
            vec.push(StudentInfo { 
                uuid:   row.get("uuid"),
                number: row.get("number"),
                fname:  row.get("first_name_hash"),
                lname:  row.get("last_name_hash") 
            });
        }
        Ok(vec)
    }

    async fn insert_encrypted(&self, data: &StudentEncrypted) -> Result<(), Error> {
        return match self.client.execute("insert into encrypteddata (UUID, encrypted) values ($1, $2)", &[&data.uuid, &data.data]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Inserting encrypted".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn update_encrypted(&self, update: &UpdateStudentEncrypted) -> Result<(), Error> {
        if update.data.is_none() {
            return Ok(())
        }
        return match self.client.execute("update encrypteddata set encrypted = $1 where UUID = $2", &[&update.uuid, &update.data.clone().unwrap()]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Updating encrypted".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn delete_encrypted(&self, uuid: &str) -> Result<(), Error> {
        return match self.client.execute("delete from encrypteddata where UUID = $1", &[&uuid]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Deleting encrypted".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn get_encrypted(&self, uuid: &str) -> Result<StudentEncrypted, Error> {
        let row = match self.client.query_one("select encrypted from encrypteddata where UUID = $1", &[&uuid]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting encrypted".to_owned(), Box::new(Error::PostgresError(t))))
        };

        Ok(StudentEncrypted {
            uuid: uuid.to_string(),
            data: row.get::<&str, &str>("encrypted").to_string(),
        })
    }

    async fn getall_encrypted(&self) -> Result<Vec<StudentEncrypted>, Error> {
        let mut vec: Vec<StudentEncrypted> = vec![];

        let rows = match self.client.query("select * from encrypteddata", &[]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting encrypted".to_owned(), Box::new(Error::PostgresError(t))))
        };

        for row in rows {
            vec.push(StudentEncrypted{
                uuid: row.get("uuid"),
                data: row.get("encrypted")
            });
        }

        Ok(vec)
    }


    async fn insert_residence(&self, user: &StudentResidence) -> Result<(), Error> {
        return match self.client.execute("insert into residencies (UUID, hall, room, wing) values ($1, $2, $3, $4)",
            &[&user.uuid, &user.hall, &user.room, &user.wing]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Inserting residence".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn update_residence(&self, update: &UpdateStudentResidence) -> Result<(), Error> {
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

        params.push(Box::new(update.uuid.to_owned()));
        let query = format!("update residencies set {} where UUID = ${}", set_clauses.join(", "), idx);

        let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref() as &(dyn ToSql + Sync)).collect();

        return match self.client.execute(&query, &param_refs[..]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Updating residence".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn delete_residence(&self, uuid: &str) -> Result<(), Error> {
        return match self.client.execute("delete from residencies where UUID = $1", &[&uuid]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Deleting residence".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn get_residence(&self, uuid: &str) -> Result<StudentResidence, Error> {
        let row = match self.client.query_one("select * from residencies where UUID = $1", &[&uuid]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting residence".to_owned(), Box::new(Error::PostgresError(t))))
        };

        Ok(StudentResidence {
            uuid: uuid.to_string(),
            hall: row.get::<&str, &str>("hall").to_string(),
            room: row.get::<&str, i32>("room"),
            wing: row.get::<&str, &str>("wing").to_string(),
        })
    }

    async fn search_residence(&self, params: &SearchStudentResidence) -> Result<Vec<StudentResidence>, Error> {
        // Dynamically build query
        let mut clauses: Vec<String> = Vec::new();
        let mut query_params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
        let mut idx = 1;

        if let Some(hall) = &params.hall {
            clauses.push(format!("hall = ${}", idx));
            query_params.push(Box::new(hall.clone()));
            idx += 1;
        }
        if let Some(room) = &params.room {
            clauses.push(format!("room = ${}", idx));
            query_params.push(Box::new(*room));
            idx += 1;
        }
        if let Some(wing) = &params.wing {
            clauses.push(format!("wing = ${}", idx));
            query_params.push(Box::new(wing.clone()));
        }

        let param_refs: Vec<&(dyn ToSql + Sync)> = query_params.iter().map(|p| p.as_ref() as &(dyn ToSql + Sync)).collect();
        
        let query_string = format!("select * from residencies {} {}",
            if clauses.len() != 0 { "where" } else {""},
            clauses.join(" and ")
        );

        let rows = match self.client.query(&query_string, &param_refs[..]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting residence".to_owned(), Box::new(Error::PostgresError(t))))
        };
        let mut vec: Vec<StudentResidence> = vec![];

        for row in rows {
            vec.push(StudentResidence{
                uuid: row.get("uuid"),
                hall: row.get::<&str, &str>("hall").to_string(),
                room: row.get::<&str, i32>("room"),
                wing: row.get::<&str, &str>("wing").to_string(),
            })
        }

        Ok(vec)
    }
}