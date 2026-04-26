use axum::async_trait;

// this holds the traits that directly interface with the database. These can be easily faked for tests.
use crate::database::Error;
use crate::types::Role;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn insert_user(&mut self, user: &FullUser) -> Result<(), Error>;
    async fn update_user(&mut self, uuid: &str, update: &UpdateUser) -> Result<(), Error>;
    async fn delete_user(&mut self, uuid: &str) -> Result<(), Error>;
    async fn get_user   (&mut self, uuid: &str) -> Result<(FullUser, Vec<TokenInfo>), Error>;
    async fn getall_user(&mut self) -> Result<Vec<FullUser>, Error>;

    async fn insert_token(&mut self, uuid: &str, hashed_token: &str, device: &str) -> Result<(), Error>;
    // async fn update_token(&mut self, uuid: &str, device: &str) -> Result<(), Error>;
    async fn delete_token(&mut self, uuid: &str, hashed_token: &str) -> Result<(), Error>;
    async fn has_token   (&mut self, uuid: &str, hashed_token: &str) -> Result<bool, Error>;
    async fn getall_token(&mut self) -> Result<Vec<TokenInfo>, Error>;
}

pub struct TokenInfo {
    pub uuid: String,
    pub hashed_token: String,
    pub device: String
}

pub struct FullUser {
    pub uuid: String,
    pub fname: String,
    pub lname: String,
    pub role: Role
}

pub struct UpdateUser {
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub role:  Option<Role>
}