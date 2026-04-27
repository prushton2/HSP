use tokio_postgres::{Client, NoTls};

use crate::{database::{DBInfo, Error}, repository::Repository};

pub mod student;
pub mod auth;


pub struct PSQLDB {
    client: Client
}

impl Repository for PSQLDB {}

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

    pub async fn init_if_uninitialized(&self) -> Result<(), Error> {
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
                wing varchar(64)
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
                signup_hash text,
                expiry bigint,

                PRIMARY KEY (UUID, token)
            );
        ").await;

        match result {
            Ok(_) => {return Ok(())},
            Err(t) => {return Err(Error::PostgresError(t))}
        };
    }
}