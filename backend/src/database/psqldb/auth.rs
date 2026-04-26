use axum::async_trait;

use crate::database::Error;
use crate::repository::AuthRepository;
use crate::repository::auth_repository::{FullUser, TokenInfo, UpdateUser};

#[async_trait]
impl AuthRepository for super::PSQLDB {
    async fn insert_user(&mut self, user: &FullUser) -> Result<(), Error> {
        let role = String::from(&user.role);
        match self.client.execute(
            "INSERT INTO Users (UUID, first_name, last_name, role) VALUES ($1, $2, $3, $4)",
            &[&user.uuid, &user.fname, &user.lname, &role]
        ).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Inserting user".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        }
    }

    async fn update_user(&mut self, uuid: &str, update: &UpdateUser) -> Result<(), Error> {
        if let Some(ref fname) = update.fname {
            match self.client.execute(
                "UPDATE Users SET first_name = $1 WHERE UUID = $2",
                &[fname, &uuid]
            ).await {
                Ok(_) => {},
                Err(t) => return Err(Error::ErrorDuring("Updating user first_name".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
            }
        }

        if let Some(ref lname) = update.lname {
            match self.client.execute(
                "UPDATE Users SET last_name = $1 WHERE UUID = $2",
                &[lname, &uuid]
            ).await {
                Ok(_) => {},
                Err(t) => return Err(Error::ErrorDuring("Updating user last_name".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
            }
        }

        if let Some(ref role) = update.role {
            let role_str = String::from(role);
            match self.client.execute(
                "UPDATE Users SET role = $1 WHERE UUID = $2",
                &[&role_str, &uuid]
            ).await {
                Ok(_) => {},
                Err(t) => return Err(Error::ErrorDuring("Updating user role".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
            }
        }

        Ok(())
    }

    async fn delete_user(&mut self, uuid: &str) -> Result<(), Error> {
        match self.client.execute(
            "DELETE FROM Users WHERE UUID = $1",
            &[&uuid]
        ).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Deleting user".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        }
    }

    async fn get_user(&mut self, uuid: &str) -> Result<(FullUser, Vec<TokenInfo>), Error> {
        let user = match self.client.query_opt(
            "SELECT * FROM Users WHERE UUID = $1",
            &[&uuid]
        ).await {
            Ok(Some(row)) => FullUser {
                uuid:  row.get::<&str, &str>("UUID").to_string(),
                fname: row.get::<&str, &str>("first_name").to_string(),
                lname: row.get::<&str, &str>("last_name").to_string(),
                role:  row.get::<&str, &str>("role").into(),
            },
            Ok(None) => return Err(Error::ErrorDuring("Getting user".to_owned(), Box::new(Error::InvalidParameter("uuid".to_owned(), uuid.to_owned())))),
            Err(t)   => return Err(Error::ErrorDuring("Getting user".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        let users_tokens = match self.client.query("SELECT * FROM Tokens WHERE uuid = $1", &[&uuid]).await {
            Ok(rows) => {
                rows.into_iter().map(|row| {
                    TokenInfo {
                        uuid: row.get("uuid"),
                        hashed_token: row.get("token"),
                        device: row.get("device")
                    }
                }).collect::<Vec<TokenInfo>>()
            },
            Err(t)   => return Err(Error::ErrorDuring("Getting user's tokens".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        };

        Ok((user, users_tokens))
    }

    async fn getall_user(&mut self) -> Result<Vec<FullUser>, Error> {
        match self.client.query("SELECT * FROM Users", &[]).await {
            Ok(rows) => Ok(rows.iter().map(|row| FullUser {
                uuid:  row.get::<&str, &str>("UUID").to_string(),
                fname: row.get::<&str, &str>("first_name").to_string(),
                lname: row.get::<&str, &str>("last_name").to_string(),
                role:  row.get::<&str, &str>("role").into(),
            }).collect()),
            Err(t) => Err(Error::ErrorDuring("Getting all users".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        }
    }

    async fn insert_token(&mut self, uuid: &str, hashed_token: &str, device: &str) -> Result<(), Error> {
        match self.client.execute(
            "INSERT INTO Tokens (UUID, token, device) VALUES ($1, $2, $3)",
            &[&uuid, &hashed_token, &device]
        ).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Inserting token".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        }
    }

    async fn delete_token(&mut self, uuid: &str, hashed_token: &str) -> Result<(), Error> {
        match self.client.execute(
            "DELETE FROM Tokens WHERE UUID = $1 AND token = $2",
            &[&uuid, &hashed_token]
        ).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Deleting token".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        }
    }

    async fn has_token(&mut self, uuid: &str, hashed_token: &str) -> Result<bool, Error> {
        match self.client.query_opt(
            "SELECT 1 FROM Tokens WHERE UUID = $1 AND token = $2",
            &[&uuid, &hashed_token]
        ).await {
            Ok(row) => Ok(row.is_some()),
            Err(t)  => Err(Error::ErrorDuring("Checking token".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        }
    }

    async fn getall_token(&mut self) -> Result<Vec<TokenInfo>, Error> {
        match self.client.query("SELECT * FROM Tokens", &[]).await {
            Ok(rows) => Ok(rows.iter().map(|row| TokenInfo {
                uuid:         row.get::<&str, &str>("UUID").to_string(),
                hashed_token: row.get::<&str, &str>("token").to_string(),
                device:       row.get::<&str, &str>("device").to_string(),
            }).collect()),
            Err(t) => Err(Error::ErrorDuring("Getting all tokens".to_owned(), Box::new(Error::PostgresError(t.code().cloned()))))
        }
    }
}