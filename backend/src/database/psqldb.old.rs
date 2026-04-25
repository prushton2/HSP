use std::collections::{HashMap, HashSet};

use axum::async_trait;
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;
use crate::database::{self, DBInfo, Error};
use crate::encryption::{self, EncryptedContents, Encryption};
use crate::types::Role;

pub struct PSQLDB {
    client: Client
}

impl PSQLDB {
    pub async fn new(dbinfo: &DBInfo) -> Self {
        let string: String = format!("host={} user={} password={} dbname={}", dbinfo.host, dbinfo.username, dbinfo.password, dbinfo.dbname);
        let new_client = match tokio_postgres::connect(&string, NoTls).await {
            Ok(t) => t,
            Err(t) => panic!("Couldnt connect to database: {}", t)
        };

        tokio::spawn(async move {
            if let Err(e) = new_client.1.await {
                eprintln!("DB connection error: {}", e);
            }
        });

        let db: Self = Self{
            client: new_client.0,
        };
        
        return db;
    }
}

#[async_trait]
impl database::Database for PSQLDB {
    fn get_encryption(&mut self) -> &mut dyn Encryption {
        return self.encryption.as_mut();
    }

    async fn init_if_uninitialized(&mut self) -> Result<(), database::Error> {
        let result = self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS EncryptedData (
                UUID varchar(36) PRIMARY KEY,
                encrypted text
            );

            CREATE TABLE IF NOT EXISTS StudentInfo (
                UUID varchar(36),
                number integer PRIMARY KEY,
                first_name_hash text,
                last_name_hash text
            );

            CREATE TABLE IF NOT EXISTS Residencies (
                UUID varchar(36) PRIMARY KEY,
                hall varchar(16),
                room integer,
                wing varchar(64),
                role varchar(64)
            );

            CREATE TABLE IF NOT EXISTS StudentActivities (
                UUID varchar(36),
                date date,
                activity text,

                PRIMARY KEY (UUID, date)
            );

            CREATE TABLE IF NOT EXISTS Activities (
                activity text,
                date date,
                staff text[8],

                PRIMARY KEY (activity, date)
            );

            CREATE TABLE IF NOT EXISTS Users (
                UUID varchar(36) PRIMARY KEY,
                first_name text,
                last_name text,
                role text
            );

            CREATE TABLE IF NOT EXISTS Tokens (
                UUID varchar(36),
                token text,
                device text,

                PRIMARY KEY (UUID, token)
            );
        ").await;

        match result {
            Ok(_) => {return Ok(())},
            Err(t) => {return Err(database::Error::PostgresError(t.code().cloned()))}
        };
    }

    async fn get_student_tables(&mut self) -> Result<(
            Vec<database::TableStudentInfo>,
            Vec<database::TableResidencies>,
            Vec<database::TableStudentActivities>,
            Vec<database::TableActivities>
        ),
        database::Error> {
        
        let rows = self.client.query("SELECT * FROM StudentInfo", &[]).await.unwrap();
        
        let student_info = rows.iter().map(|row| {
            database::TableStudentInfo {
                uuid: row.get::<&str, &str>("UUID").to_string(),
                number: row.get::<&str, i32>("number") as i32,
            }
        }).collect();

        let rows = self.client.query("SELECT * FROM Residencies", &[]).await.unwrap();
        
        let residencies = rows.iter().map(|row| {
            database::TableResidencies {
                uuid: row.get::<&str, &str>("UUID").to_string(),
                hall: row.get::<&str, &str>("hall").to_string(),
                room: row.get::<&str, i32>("room"),
                wing: row.get::<&str, &str>("wing").to_string(),
                role: row.get::<&str, &str>("role").to_string(),
            }
        }).collect();

        let rows = self.client.query("SELECT * FROM StudentActivities", &[]).await.unwrap();
        
        let student_activities = rows.iter().map(|row| {
            database::TableStudentActivities {
                uuid: row.get::<&str, &str>("UUID").to_string(),
                date: row.get::<&str, chrono::NaiveDate>("date"),
                activity: row.get::<&str, &str>("activity").to_string(),
            }
        }).collect();

        let rows = self.client.query("SELECT * FROM Activities", &[]).await.unwrap();
        
        let activities = rows.iter().map(|row| {
            database::TableActivities {
                activity: row.get::<&str, &str>("activity").to_string(),
                date: row.get::<&str, chrono::NaiveDate>("date"),
                staff: row.get::<&str, Vec<String>>("staff"),
            }
        }).collect();

        Ok((student_info, residencies, student_activities, activities))
    }

    async fn create_student(&mut self, user: &crate::endpoints::student::CreateUser) -> Result<(), Error> {
        let id = Uuid::new_v4().to_string();

        match self.client.execute("INSERT INTO StudentInfo (uuid, number) VALUES ($1, $2)", &[&id, &user.number]).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Insert into StudentInfo".to_string(), Box::new(Error::PostgresError(t.code().cloned()))))
        }

        let encrypted_data = self.encryption.encrypt(&EncryptedContents { first_name: user.fname.clone(), last_name: user.lname.clone(), pronouns: user.pronouns.clone() });

        match self.client.execute("insert into EncryptedData (uuid, encrypted) values ($1, $2)", &[&id, &encrypted_data]).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Insert into EncryptedData".to_string(), Box::new(Error::PostgresError(t.code().cloned()))))
        }

        match self.client.execute("insert into residencies (uuid, hall, room, wing, role) values ($1, $2, $3, $4, $5)", &[&id, &user.hall, &user.room, &user.wing, &user.role]).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Insert into Residencies".to_string(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        Ok(())
    }

    async fn edit_student(&mut self, uuid: &str, field: &str, new_value: &database::FieldValue) -> Result<(), Error> {
        enum OpType {
            IntField(String),
            StringField(String),
            EncryptedField
        }
        let valid_fields: HashMap<&str, OpType> = [
            ("number", OpType::IntField("StudentInfo".to_owned())),
            ("hall", OpType::StringField("Residencies".to_owned())),
            ("room", OpType::IntField("Residencies".to_owned())),
            ("wing", OpType::StringField("Residencies".to_owned())),
            ("role", OpType::StringField("Residencies".to_owned())),
            ("first name", OpType::EncryptedField),
            ("last name", OpType::EncryptedField),
            ("pronouns", OpType::EncryptedField),
        ].into();

        if valid_fields.get(&field).is_none() {
            return Err(Error::InvalidParameter("Invalid field".to_string(), field.to_string()));
        }

        match &valid_fields.get(&field).unwrap() {
            OpType::IntField(table) => {
                let v: i32 = match new_value {
                    database::FieldValue::I32(t) => *t,
                    _ => return Err(Error::InvalidParameter("New value must be an int".to_owned(), "".to_owned()))
                };

                let query = format!("update {} set {} = $1 where UUID = $2", table, field);
                match self.client.execute(&query, &[&v, &uuid]).await {
                    Ok(_) => {},
                    Err(t) => return Err(Error::ErrorDuring("Updating Integer Field".to_string(), Box::new(Error::PostgresError(t.code().cloned()))))
                };
            },
            OpType::StringField(table) => {
                let v: &str = match new_value {
                    database::FieldValue::Str(t) => t,
                    _ => return Err(Error::InvalidParameter("New value must be a string".to_owned(), "".to_owned()))
                };
                
                let query = format!("update {} set {} = $1 where UUID = $2", table, field);
                match self.client.execute(&query, &[&v, &uuid]).await {
                    Ok(_) => {},
                    Err(t) => return Err(Error::ErrorDuring("Updating Integer Field".to_string(), Box::new(Error::PostgresError(t.code().cloned()))))
                };
            },
            OpType::EncryptedField => {
                let v: String = match new_value {
                    database::FieldValue::Str(t) => t.to_string(),
                    _ => return Err(Error::InvalidParameter("New value must be a string".to_owned(), "".to_owned()))
                };

                let row = match self.client.query_one("select * from encrypteddata where uuid = $1", &[&uuid]).await {
                    Ok(t) => t,
                    Err(t) => return Err(Error::ErrorDuring("Fetching encrypted data".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
                };

                let mut data = match self.encryption.decrypt(row.get("encrypted")) {
                    Some(t) => t,
                    None => encryption::EncryptedContents {
                        first_name: "".to_string(),
                        last_name: "".to_string(),
                        pronouns: "".to_string(),
                    }
                };

                match field {
                    "first name" => {data.first_name = v;},
                    "last name" =>  {data.last_name  = v;},
                    "pronouns" =>   {data.pronouns   = v;},
                    _ => return Err(Error::InvalidParameter("field must be one of `first name | last name | pronouns`".to_owned(), field.to_owned()))
                }

                let encrypted = self.encryption.encrypt(&data);

                match self.client.execute("update encrypteddata set encrypted = $1 where uuid = $2", &[&encrypted, &uuid]).await {
                    Ok(_) => {},
                    Err(t) => return Err(Error::ErrorDuring("Writing encrypted data".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
                };

            }
        }

        Ok(())
    }

    async fn get_student(&mut self, uuid: &str, decrypt: bool) -> Result<database::AllStudentInfo, Error> {
        let mut student = database::AllStudentInfo::default();

        let studentinfo = self.client.query_one("SELECT * FROM StudentInfo WHERE uuid = $1", &[&uuid]).await.unwrap();
        
        student.info.number = studentinfo.get("number");
        student.info.uuid = studentinfo.get("uuid");

        let residency = self.client.query_one("SELECT * FROM Residencies WHERE uuid = $1", &[&uuid]).await.unwrap();
        
        student.residence.uuid = residency.get::<&str, &str>("UUID").to_string();
        student.residence.hall = residency.get::<&str, &str>("hall").to_string();
        student.residence.room = residency.get::<&str, i32>("room");
        student.residence.wing = residency.get::<&str, &str>("wing").to_string();
        student.residence.role = residency.get::<&str, &str>("role").to_string();

        if !decrypt {
            return Ok(student);
        }

        let encrypted = self.client.query_one("SELECT encrypted FROM encrypteddata WHERE uuid = $1", &[&uuid]).await.unwrap();

        let decrypted = match self.encryption.decrypt(encrypted.get("encrypted")) {
            Some(t) => t,
            None => EncryptedContents::default()
        };

        student.first_name = decrypted.first_name;
        student.last_name = decrypted.last_name;
        student.pronouns = decrypted.pronouns;

        Ok(student)
    }

    async fn delete_student(&mut self, uuid: &str) -> Result<(), Error> {
        let tables = ["encrypteddata", "studentinfo", "residencies", "studentactivities"];

        for table in tables {
            let statement = format!("DELETE FROM {} WHERE uuid = $1", table);

            match self.client.execute(&statement, &[&uuid]).await {
                Ok(_) => (),
                Err(t) => return Err(Error::ErrorDuring(format!("Deleting from {}", table), Box::new(Error::PostgresError(t.code().cloned()))))
            }
        }

        Ok(())
    }

    async fn create_user(&mut self, first_name: &str, last_name: &str, role: Role, device: &str) -> Result<Uuid, Error> {
        let uuid = Uuid::new_v4();
        let str_role: String = String::from(&role);

        match self.client.execute("insert into Users (UUID, first_name, last_name, role) values ($1, $2, $3, $4)", &[&uuid.to_string(), &first_name, &last_name, &str_role]).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting User".to_string(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        
        Ok(uuid)
    }
    
    async fn grant_access(&mut self, uuid: &str, device: &str) -> Result<String, Error> {
        let token = self.encryption.random_string(32);
    
        match self.client.execute("insert into Tokens (UUID, token, device) values ($1, $2, $3)", &[&uuid.to_string(), &self.encryption.hash(&token, ""), &device]).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting Token".to_string(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        Ok(token)
    }

    async fn get_user(&mut self, token: &str) -> Option<UserInfo> {
        let mut user_info = UserInfo {
            first_name: "".to_string(),
            last_name: "".to_string(),
            role: Role::Staff,
            accessed_from: "".to_string(),
        };

        let row = match self.client.query_opt("select * from tokens where token = $1", &[&self.encryption.hash(&token, "")]).await.unwrap() {
            Some(t) => t,
            None => return None
        };

        let uuid = row.get::<&str, &str>("uuid");

        let user = match self.client.query_opt("select * from users where uuid = $1", &[&uuid]).await.unwrap() {
            Some(t) => t,
            None => return None
        };

        user_info.first_name = user.get("first_name");
        user_info.last_name = user.get("last_name");
        user_info.role = user.get::<&str, &str>("role").into();
        user_info.accessed_from = row.get("device");

        Some(user_info)
    }


    async fn edit_user(&mut self, uuid: &str, field: &str, new_value: &database::FieldValue) -> Result<(), Error> {
        if !HashSet::from(["first_name", "last_name", "role"]).contains(field) {
            return Err(Error::InvalidParameter("field".to_owned(), "".to_owned()));
        }

        let statement = format!("update Users set {} = $1 where uuid = $2", field);
        match self.client.execute(&statement, &[&new_value, &uuid]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::PostgresError(t.code().cloned()))
        }
    }
    
    async fn delete_user(&mut self, uuid: &str) -> Result<(), Error> {
        
        match self.client.execute("delete from Users where uuid = $1", &[&uuid]).await {
            Ok(_) => {},
            Err(t) => return Err(Error::PostgresError(t.code().cloned()))
        };
        
        match self.client.execute("delete from tokens where uuid = $1", &[&uuid]).await {
            Ok(_) => {},
            Err(t) => return Err(Error::PostgresError(t.code().cloned()))
        };

        Ok(())
    }
}

