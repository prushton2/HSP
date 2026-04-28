// The service handles the actual logic to doing stuff to the database.

use chrono::Utc;
use tokio::sync::Mutex;
use uuid::Uuid;
use axum_extra::extract::CookieJar;

use crate::SIGNUP_HASH_EXPIRY;
use crate::encryption::Encryption;

use crate::types::Error;

use crate::repository::Repository;
use crate::repository::auth_repository::{FullUser, UpdateUser};
use crate::types::Role;

pub struct AuthService {
    repo: Box<dyn Repository>,
    encryption: Box<dyn Encryption>,
    signup_mutex: Mutex<bool> // load bearing drywall that determines if someone is signing up (one signup at a time)
}

impl AuthService {
    pub fn new(repo: Box<dyn Repository>, encryption: Box<dyn Encryption>) -> Self {
        Self {
            repo: repo,
            encryption: encryption,
            signup_mutex: Mutex::new(false),
        }
    }

    pub async fn is_authenticated(&self, jar: &CookieJar, permission: &Role, action: &str) -> Option<FullUser> {
        let token = match jar.get("token") {
            Some(t) => t.value(),
            None => return None
        };

        let hashed_token = self.encryption.hash(token, "");

        let token_entry = match self.repo.get_token(&hashed_token).await {
            Ok(t) => t,
            Err(_) => return None
        };

        let (user_entry, _) = match self.repo.get_user(&token_entry.uuid).await {
            Ok(t) => t,
            Err(_) => return None
        };

        if token_entry.expiry < Utc::now().timestamp() {
            log::error!("[{}]: token expired for {} {}", user_entry.uuid, user_entry.fname, user_entry.lname);
            return None
        }

        let success = user_entry.role >= *permission;

        log::info!("[{}]: {} access to {}", user_entry.uuid, if success { "granted" } else { "denied" }, action);

        match success {
            true  => Some(user_entry),
            false => None
        }
    }

    pub async fn create_user(&self, user: &FullUser) -> Result<String, Error> {
        let new_user = FullUser {
            fname: user.fname.clone(),
            lname: user.lname.clone(),
            uuid: Uuid::new_v4().to_string(),
            role: user.role.clone()
        };

        let signup_hash = self.encryption.random_string(32);

        match self.repo.insert_user(&new_user).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting user".to_owned(), Box::new(t)))
        };

        // this placeholder token ensures that the entry can be uniquely identified. It is NOT used in any authentication
        let placeholder_token = self.encryption.random_string(32);

        match self.repo.insert_token(&new_user.uuid, &placeholder_token, &signup_hash, Utc::now().timestamp() + 86400).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting token".to_owned(), Box::new(t)))
        };

        Ok(signup_hash)
    }

    pub async fn signup(&self, signup_hash: &str) -> Result<String, Error> {
        let _lock = self.signup_mutex.lock().await;

        if signup_hash == "" {
            return Err(Error::InvalidParameter("signup_hash".to_owned(), "".to_owned()));
        }

        let token = match self.repo.get_token_hash(signup_hash).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Fetching token for signup".to_owned(), Box::new(t)))
        };

        if token.expiry < Utc::now().timestamp() {
            let _ = self.repo.delete_token(&token.uuid, &token.token).await;
            return Err(Error::ExpiredError)
        }

        let unhashed_token = self.encryption.random_string(32);
        let hashed_token = self.encryption.hash(&unhashed_token, "");

        let (user, _) = match self.repo.get_user(&token.uuid).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Fetching User".to_owned(), Box::new(t)))
        };

        match self.repo.update_token(&token.uuid, &token.token, Some(hashed_token.as_str()), Some(""), Some(Utc::now().timestamp() + 3024000)).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Updating token".to_owned(), Box::new(t)))
        };

        log::info!("[{}] user signed up: {} {}", token.uuid, user.fname, user.lname);

        Ok(unhashed_token)
    }

    pub async fn update_user(&self, uuid: &str, update: &UpdateUser) -> Result<(), Error> {
        match self.repo.update_user(&uuid, &update).await {
            Ok(()) => Ok(()),
            Err(t) => return Err(Error::ErrorDuring("Updating User".to_owned(), Box::new(t)))
        }
    }

    pub async fn delete_user(&self, uuid: &str) -> Result<(), Error>{
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

    pub async fn revoke_tokens(&self, uuid: &str) -> Result<(), Error> {
        match self.repo.delete_tokens(uuid).await {
            Ok(_) => {Ok(())},
            Err(t) => Err(Error::ErrorDuring("Deleting Tokens".to_owned(), Box::new(t)))
        }
    }

    pub async fn grant_token(&self, uuid: &str) -> Result<String, Error>{
        let token = self.encryption.random_string(32);
        let signup_hash = self.encryption.random_string(32);

        match self.repo.insert_token(uuid, &token, &signup_hash, SIGNUP_HASH_EXPIRY).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting token".to_owned(), Box::new(t)))
        };

        Ok(signup_hash)
    }
    
}