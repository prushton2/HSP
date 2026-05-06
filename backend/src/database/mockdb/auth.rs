use axum::async_trait;

use crate::types::Error;
use crate::repository::AuthRepository;
use crate::repository::auth_repository::{User, Token, UpdateUser};

#[async_trait]
impl AuthRepository for super::MockDB {
    async fn insert_user(&self, user: &User) -> Result<(), Error> {
    }

    async fn update_user(&self, uuid: &str, update: &UpdateUser) -> Result<(), Error> {
    }

    async fn delete_user(&self, uuid: &str) -> Result<(), Error> {
    }

    async fn get_user(&self, uuid: &str) -> Result<(User, Vec<Token>), Error> {
    }

    async fn getall_user(&self) -> Result<Vec<User>, Error> {
    }

    async fn insert_token(&self, uuid: &str, plain_token: &str, signup_hash: &str, expiry: i64) -> Result<(), Error> {
    }

    async fn update_token(&self, uuid: &str, old_token: &str, new_token: Option<&str>, new_signup_hash: Option<&str>, new_expiry: Option<i64>) -> Result<(), Error> {
    }

    async fn delete_token(&self, uuid: &str, hashed_token: &str) -> Result<(), Error> {
    }

    async fn delete_tokens(&self, uuid: &str) -> Result<(), Error> {
    }

    async fn get_token_hash(&self, signup_hash: &str) -> Result<Token, Error> {
    }

    async fn get_token(&self, token: &str) -> Result<Token, Error> {
    }


    async fn getall_token(&self) -> Result<Vec<Token>, Error> {
    }
}