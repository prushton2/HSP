use axum::async_trait;

use crate::types::Error;
use crate::repository::AuthRepository;
use crate::repository::auth_repository::{User, Token, UpdateUser};

#[async_trait]
impl AuthRepository for super::MockDB {
    async fn insert_user(&self, user: &User) -> Result<(), Error> {
        let mut map = self.users.lock().unwrap();
        map.insert(user.uuid.clone(), User {
            uuid:  user.uuid.clone(),
            fname: user.fname.clone(),
            lname: user.lname.clone(),
            role:  user.role.clone(),
        });
        Ok(())
    }

    async fn update_user(&self, uuid: &str, update: &UpdateUser) -> Result<(), Error> {
        let mut map = self.users.lock().unwrap();
        let entry = map.get_mut(uuid)
            .ok_or(Error::InvalidParameter("uuid".to_owned(), uuid.to_owned()))?;

        if let Some(ref fname) = update.fname {
            entry.fname = fname.clone();
        }
        if let Some(ref lname) = update.lname {
            entry.lname = lname.clone();
        }
        if let Some(ref role) = update.role {
            entry.role = role.clone();
        }
        Ok(())
    }

    async fn delete_user(&self, uuid: &str) -> Result<(), Error> {
        let mut map = self.users.lock().unwrap();
        map.remove(uuid);

        // Also clean up associated tokens
        let mut tokens = self.tokens.lock().unwrap();
        tokens.retain(|_, t| t.uuid != uuid);
        Ok(())
    }

    async fn get_user(&self, uuid: &str) -> Result<(User, Vec<Token>), Error> {
        let users = self.users.lock().unwrap();
        let user = users.get(uuid)
            .map(|u| User {
                uuid:  u.uuid.clone(),
                fname: u.fname.clone(),
                lname: u.lname.clone(),
                role:  u.role.clone(),
            })
            .ok_or(Error::InvalidParameter("uuid".to_owned(), uuid.to_owned()))?;

        let tokens = self.tokens.lock().unwrap();
        let user_tokens: Vec<Token> = tokens.values()
            .filter(|t| t.uuid == uuid)
            .map(|t| Token {
                uuid:        t.uuid.clone(),
                token:       t.token.clone(),
                signup_hash: t.signup_hash.clone(),
                expiry:      t.expiry,
            })
            .collect();

        Ok((user, user_tokens))
    }

    async fn getall_user(&self) -> Result<Vec<User>, Error> {
        let map = self.users.lock().unwrap();
        Ok(map.values()
            .map(|u| User {
                uuid:  u.uuid.clone(),
                fname: u.fname.clone(),
                lname: u.lname.clone(),
                role:  u.role.clone(),
            })
            .collect())
    }

    async fn insert_token(&self, uuid: &str, plain_token: &str, signup_hash: &str, expiry: i64) -> Result<(), Error> {
        let mut map = self.tokens.lock().unwrap();
        let key = format!("{}:{}", uuid, plain_token);
        map.insert(key, Token {
            uuid:        uuid.to_owned(),
            token:       plain_token.to_owned(),
            signup_hash: signup_hash.to_owned(),
            expiry,
        });
        Ok(())
    }

    async fn update_token(&self, uuid: &str, old_token: &str, new_token: Option<&str>, new_signup_hash: Option<&str>, new_expiry: Option<i64>) -> Result<(), Error> {
        let mut map = self.tokens.lock().unwrap();
        let old_key = format!("{}:{}", uuid, old_token);

        let entry = map.remove(&old_key)
            .ok_or(Error::InvalidParameter("token".to_owned(), old_token.to_owned()))?;

        let updated_token = new_token.unwrap_or(&entry.token).to_owned();
        let updated_hash = new_signup_hash.unwrap_or(&entry.signup_hash).to_owned();
        let updated_expiry = new_expiry.unwrap_or(entry.expiry);

        let new_key = format!("{}:{}", uuid, updated_token);
        map.insert(new_key, Token {
            uuid:        uuid.to_owned(),
            token:       updated_token,
            signup_hash: updated_hash,
            expiry:      updated_expiry,
        });
        Ok(())
    }

    async fn delete_token(&self, uuid: &str, hashed_token: &str) -> Result<(), Error> {
        let mut map = self.tokens.lock().unwrap();
        let key = format!("{}:{}", uuid, hashed_token);
        map.remove(&key);
        Ok(())
    }

    async fn delete_tokens(&self, uuid: &str) -> Result<(), Error> {
        let mut map = self.tokens.lock().unwrap();
        map.retain(|_, t| t.uuid != uuid);
        Ok(())
    }

    async fn get_token_hash(&self, signup_hash: &str) -> Result<Token, Error> {
        let map = self.tokens.lock().unwrap();
        map.values()
            .find(|t| t.signup_hash == signup_hash)
            .map(|t| Token {
                uuid:        t.uuid.clone(),
                token:       t.token.clone(),
                signup_hash: t.signup_hash.clone(),
                expiry:      t.expiry,
            })
            .ok_or(Error::InvalidParameter("signup_hash".to_owned(), signup_hash.to_owned()))
    }

    async fn get_token(&self, token: &str) -> Result<Token, Error> {
        let map = self.tokens.lock().unwrap();
        map.values()
            .find(|t| t.token == token)
            .map(|t| Token {
                uuid:        t.uuid.clone(),
                token:       t.token.clone(),
                signup_hash: t.signup_hash.clone(),
                expiry:      t.expiry,
            })
            .ok_or(Error::InvalidParameter("token".to_owned(), token.to_owned()))
    }

    async fn getall_token(&self) -> Result<Vec<Token>, Error> {
        let map = self.tokens.lock().unwrap();
        Ok(map.values()
            .map(|t| Token {
                uuid:        t.uuid.clone(),
                token:       t.token.clone(),
                signup_hash: t.signup_hash.clone(),
                expiry:      t.expiry,
            })
            .collect())
    }
}