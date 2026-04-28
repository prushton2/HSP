use axum::async_trait;
use serde::{Deserialize, Serialize};

// this holds the traits that directly interface with the database. These can be easily faked for tests.
use crate::types::Error;
use crate::types::Role;

#[allow(dead_code)]
#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn insert_user(&self, user: &FullUser) -> Result<(), Error>;
    async fn update_user(&self, uuid: &str, update: &UpdateUser) -> Result<(), Error>;
    async fn delete_user(&self, uuid: &str) -> Result<(), Error>;
    async fn get_user   (&self, uuid: &str) -> Result<(FullUser, Vec<TokenInfo>), Error>;
    async fn getall_user(&self) -> Result<Vec<FullUser>, Error>;

    async fn insert_token(&self, uuid: &str, plain_token: &str, signup_hash: &str, expiry: i64) -> Result<(), Error>;
    async fn update_token(&self, uuid: &str, old_token: &str, new_token: Option<&str>, new_signup_hash: Option<&str>, new_expiry: Option<i64>) -> Result<(), Error>;
    async fn delete_token(&self, uuid: &str, token: &str) -> Result<(), Error>;
    async fn get_token   (&self, token: &str) -> Result<TokenInfo, Error>; 
    async fn getall_token(&self) -> Result<Vec<TokenInfo>, Error>;
    
    async fn get_token_hash(&self, signup_hash: &str) -> Result<TokenInfo, Error>;
    async fn delete_tokens (&self, uuid: &str) -> Result<(), Error>;
}

// Heres the signup flow:
// Owner creates a user
// backend inserts the user and creates a token
// inserted token has the uuid of the user, the plain token, and a hash for linking someone to sign up.
// the backend returns the link hash
// the user clicks on a link with the link hash
// the backend removes the link hash from the database, returns the token, and hashes the token stored.
// the user's browser saves the returned (uhashed) token as an httponly cookie, and can now access the site.


#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    pub uuid: String,
    pub token: String,
    pub signup_hash: String,
    pub expiry: i64
}

#[derive(Serialize, Deserialize)]
pub struct FullUser {
    pub uuid: String,
    pub fname: String,
    pub lname: String,
    pub role: Role
}
#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub role:  Option<Role>
}