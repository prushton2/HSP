use postgres::{Client, NoTls};
use crate::database::{self, DBInfo};

pub struct PSQLDB {
    client: Client
}

impl PSQLDB {
    pub fn new(dbinfo: &DBInfo) -> Self {
        let string: String = format!("host={} user={} password={} dbname={}", dbinfo.host, dbinfo.username, dbinfo.password, dbinfo.dbname);
        let new_client = Client::connect(&string, NoTls).unwrap();

        let db: Self = Self{
            client: new_client,
        };
        
        return db;
    }
}

impl database::Database for PSQLDB {
    fn init_if_uninitialized(&mut self) -> Result<(), database::Error> {
        let result = self.client.batch_execute("
            CREATE TABLE IF NOT EXISTS EncryptedData (
                UUID varchar(36) PRIMARY KEY,
                encrypted text
            );

            CREATE TABLE IF NOT EXISTS StudentInfo (
                UUID varchar(36) PRIMARY KEY,
                number integer,
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

                PRIMARY KEY (activity date)
            );
        ");
        match result {
            Ok(_) => {return Ok(())},
            Err(t) => {return Err(database::Error::PostgresError(t.code().cloned()))}
        };
    }
}