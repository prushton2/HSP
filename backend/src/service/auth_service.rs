// The service handles the actual logic to doing stuff to the database.

use chrono::Utc;
use uuid::Uuid;
use axum_extra::extract::CookieJar;

use crate::SIGNUP_HASH_EXPIRY;
use crate::encryption::Encryption;

use crate::database::Error;

use crate::repository::Repository;
use crate::repository::auth_repository::{FullUser, UpdateUser};
use crate::types::Role;

// #[derive(Clone)]
pub struct AuthService {
    repo: Box<dyn Repository>,
    encryption: Box<dyn Encryption>
}

impl AuthService {
    pub fn new(repo: Box<dyn Repository>, encryption: Box<dyn Encryption>) -> Self {
        Self {
            repo: repo,
            encryption: encryption
        }
    }

    pub async fn is_authenticated(&mut self, jar: &CookieJar, permission: &Role, action: &str) -> bool {
        match jar.get("override") {
            Some(t) => {
                if t.value() == std::env::var("OVERRIDE_PASSWORD").expect("No override password set") {
                    return true
                }
            },
            None => {}
        };

        let token = match jar.get("token") {
            Some(t) => t.value(),
            None => return false
        };

        let hashed_token = self.encryption.hash(token, "");

        let token_entry = match self.repo.get_token(&hashed_token).await {
            Ok(t) => t,
            Err(_) => return false
        };

        let (user_entry, _) = match self.repo.get_user(&token_entry.uuid).await {
            Ok(t) => t,
            Err(_) => return false
        };

        let success = user_entry.role >= *permission;

        println!("{} {} ({}) was {} access to {}", user_entry.fname, user_entry.lname, user_entry.uuid, if success { "granted" } else { "denied" }, action);

        success
    }

    pub async fn create_user(&mut self, user: &FullUser) -> Result<String, Error> {
        let new_user = FullUser {
            fname: user.fname.clone(),
            lname: user.lname.clone(),
            uuid: Uuid::new_v4().to_string(),
            role: user.role.clone()
        };

        let token = self.encryption.random_string(32);
        let signup_hash = self.encryption.random_string(32);

        match self.repo.insert_user(&new_user).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting user".to_owned(), Box::new(t)))
        };

        match self.repo.insert_token(&new_user.uuid, &token, &signup_hash, 86400).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting token".to_owned(), Box::new(t)))
        };

        Ok(signup_hash)
    }

    pub async fn signup(&mut self, signup_hash: &str) -> Result<String, Error> {
        if signup_hash == "" {
            return Err(Error::InvalidParameter("signup_hash".to_owned(), "".to_owned()));
        }

        let token = match self.repo.get_token_hash(signup_hash).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Fetching token for singup".to_owned(), Box::new(t)))
        };

        if token.expiry < Utc::now().timestamp() {
            let _ = self.repo.delete_token(&token.uuid, &token.token).await;
            return Err(Error::ExpiredError)
        }

        let hashed_token = self.encryption.hash(&token.token.as_str(), "");

        match self.repo.update_token(&token.uuid, &token.token, Some(hashed_token.as_str()), Some(""), Some(3024000)).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Updating token".to_owned(), Box::new(t)))
        };

        Ok(token.token)
    }

    pub async fn update_user(&mut self, uuid: &str, update: &UpdateUser) -> Result<(), Error> {
        match self.repo.update_user(&uuid, &update).await {
            Ok(()) => Ok(()),
            Err(t) => return Err(Error::ErrorDuring("Updating User".to_owned(), Box::new(t)))
        }
    }

    pub async fn delete_user(&mut self, uuid: &str) -> Result<(), Error>{
        match self.repo.delete_user(uuid).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Deleting User".to_owned(), Box::new(t)))
        };

        match self.repo.delete_tokens(uuid).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Deleting Tokens".to_owned(), Box::new(t)))
        };

        Ok(())
    }

    pub async fn revoke_tokens(&mut self, uuid: &str) -> Result<(), Error> {
        match self.repo.delete_tokens(uuid).await {
            Ok(_) => {Ok(())},
            Err(t) => Err(Error::ErrorDuring("Deleting Tokens".to_owned(), Box::new(t)))
        }
    }

    pub async fn grant_token(&mut self, uuid: &str) -> Result<String, Error>{
        let token = self.encryption.random_string(32);
        let signup_hash = self.encryption.random_string(32);

        match self.repo.insert_token(uuid, &token, &signup_hash, SIGNUP_HASH_EXPIRY).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting token".to_owned(), Box::new(t)))
        };

        Ok(signup_hash)
    }
    
}