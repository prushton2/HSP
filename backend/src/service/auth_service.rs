// The service handles the actual logic to doing stuff to the database.

use uuid::Uuid;

use crate::encryption::Encryption;

use crate::database::Error;

use crate::repository::Repository;
use crate::repository::auth_repository::{FullUser, UpdateUser};

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

        match self.repo.insert_token(&new_user.uuid, &token, &signup_hash).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting token".to_owned(), Box::new(t)))
        };

        Ok(signup_hash)
    }

    pub async fn signup(&mut self, signup_hash: &str) -> Result<String, Error> {
        if signup_hash == "" {
            return Err(Error::InvalidParameter("signup_hash".to_owned(), "".to_owned()));
        }

        let (token, uuid) = match self.repo.get_token(signup_hash).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Fetching token for singup".to_owned(), Box::new(t)))
        };

        let hashed_token = self.encryption.hash(token.as_str(), "");

        match self.repo.update_token(&uuid, &token, Some(hashed_token.as_str()), Some("")).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Updating token".to_owned(), Box::new(t)))
        };

        Ok(token)
    }

    pub async fn update_user(&mut self, uuid: &str, update: &UpdateUser) -> Result<(), Error> {
        match self.repo.update_user(&uuid, &update).await {
            Ok(()) => Ok(()),
            Err(t) => return Err(Error::ErrorDuring("Updating User".to_owned(), Box::new(t)))
        }
    }
}