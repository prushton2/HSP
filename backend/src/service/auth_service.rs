// The service handles the actual logic to doing stuff to the database.

use uuid::Uuid;

use crate::encryption::Encryption;

use crate::database::Error;

use crate::repository::Repository;
use crate::repository::auth_repository::FullUser;

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

    pub async fn create_user(&mut self, user: &FullUser, device: &str) -> Result<String, Error> {
        let new_user = FullUser {
            fname: user.fname.clone(),
            lname: user.lname.clone(),
            uuid: Uuid::new_v4().to_string(),
            role: user.role.clone()
        };

        let token = self.encryption.random_string(32);
        let hashed_token = self.encryption.hash(&token, "");

        match self.repo.insert_user(&new_user).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting user".to_owned(), Box::new(t)))
        };

        match self.repo.insert_token(&new_user.uuid, &hashed_token, &device).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Inserting token".to_owned(), Box::new(t)))
        };

        Ok(token)
    }
}