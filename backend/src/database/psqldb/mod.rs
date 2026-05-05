use axum::async_trait;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, GenericClient, NoTls, Row, Transaction};

use crate::types::Error;
use crate::{database::DBInfo, repository::Repository};

pub mod activities;
pub mod student;
pub mod auth;

// #[async_trait]
// pub trait DbExecutor: Send + Sync {
//     async fn execute(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, tokio_postgres::Error>;
//     async fn batch_execute(&self, query: &str) -> Result<(), tokio_postgres::Error>;
//     async fn query(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, tokio_postgres::Error>;
//     async fn query_one(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, tokio_postgres::Error>;
//     async fn query_opt(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, tokio_postgres::Error>;
//     async fn transaction(&self) -> Result<Transaction<'_>, tokio_postgres::Error>;
// }

pub struct PSQLDB {
    client: Client,
    // in_transaction: bool
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
            client: new_client.0
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
                Student varchar(36),
                Activity varchar(36),

                PRIMARY KEY (Student, Activity)
            );

            CREATE TABLE IF NOT EXISTS Activities (
                uuid varchar(36) PRIMARY KEY,
                name text,
                staff text[8],
                dates bigint[32]
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

// #[async_trait]
// impl DbExecutor for tokio_postgres::Client {
//     async fn execute(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, tokio_postgres::Error> {
//         self.execute(query, params).await
//     }
//     async fn batch_execute(&self, query: &str) -> Result<(), tokio_postgres::Error> {
//         self.batch_execute(query).await
//     }
//     async fn query(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, tokio_postgres::Error> {
//         self.query(query, params).await
//     }
//     async fn query_one(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, tokio_postgres::Error> {
//         self.query_one(query, params).await
//     }
//     async fn query_opt(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, tokio_postgres::Error> {
//         self.query_opt(query, params).await
//     }
//     async fn transaction(&self) -> Result<Transaction<'_>, tokio_postgres::Error> {
//         self.transaction().await
//     }
// }

// // Transaction borrows the Client, so we need the lifetime
// #[async_trait]
// impl<'a> DbExecutor for tokio_postgres::Transaction<'a> {
//     async fn execute(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, tokio_postgres::Error> {
//         self.execute(query, params).await
//     }
//     async fn batch_execute(&self, query: &str) -> Result<(), tokio_postgres::Error> {
//         self.batch_execute(query).await
//     }
//     async fn query(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, tokio_postgres::Error> {
//         self.query(query, params).await
//     }
//     async fn query_one(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, tokio_postgres::Error> {
//         self.query_one(query, params).await
//     }
//     async fn query_opt(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, tokio_postgres::Error> {
//         self.query_opt(query, params).await
//     }
//     async fn transaction(&self) -> Result<Transaction<'_>, tokio_postgres::Error> {
//         panic!("Cannot nest transactions")
//     }
// }