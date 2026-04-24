use std::collections::{HashMap, HashSet};

use axum::async_trait;
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;
use crate::database::{self, DBInfo, Error};
use crate::encryption::{self, Encryption};

pub struct PSQLDB {
    client: Client,
    encryption: Box<dyn Encryption>
}

impl PSQLDB {
    pub async fn new(dbinfo: &DBInfo, encryption: Box<dyn Encryption>) -> Self {
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
            encryption: encryption
        };
        
        return db;
    }
}

#[async_trait]
impl database::Database for PSQLDB {
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

    async fn create_student(&mut self, user: &crate::endpoints::admin::CreateUser) -> Result<(), Error> {
        let id = Uuid::new_v4().to_string();

        match self.client.execute("INSERT INTO StudentInfo (uuid, number) VALUES ($1, $2)", &[&id, &user.number]).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Insert into StudentInfo".to_string(), Box::new(Error::PostgresError(t.code().cloned()))))
        }

        match self.client.execute("insert into EncryptedData values ($1, $2)", &[&id, &format!("{},{}", user.fname, user.lname)]).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Insert into EncryptedData".to_string(), Box::new(Error::PostgresError(t.code().cloned()))))
        }

        match self.client.execute("insert into residencies values ($1, $2, $3, $4, $5)", &[&id, &user.hall, &user.room, &user.wing, &user.role]).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Insert into Residencies".to_string(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        Ok(())
    }

    async fn edit_user(&mut self, uuid: &str, field: &str, new_value: &database::FieldValue) -> Result<(), Error> {
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

                match self.client.execute("update $1 set $2 = $3 where UUID = $4", &[&table, &field, &v, &uuid]).await {
                    Ok(_) => {},
                    Err(t) => return Err(Error::ErrorDuring("Updating Integer Field".to_string(), Box::new(Error::PostgresError(t.code().cloned()))))
                };
            },
            OpType::StringField(table) => {
                let v: &str = match new_value {
                    database::FieldValue::Str(t) => t,
                    _ => return Err(Error::InvalidParameter("New value must be a string".to_owned(), "".to_owned()))
                };

                match self.client.execute("update $1 set $2 = $3 where UUID = $4", &[&table, &field, &v, &uuid]).await {
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
}