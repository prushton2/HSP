use std::collections::HashMap;
use std::sync::Mutex;

use crate::repository::activities_repository::{Activity, ActivityBind};
use crate::repository::auth_repository::{User, Token};
use crate::repository::student_repository::{StudentEncrypted, StudentResidence, StudentInfo};
use crate::types::Error;
use crate::repository::Repository;

pub mod activities;
pub mod student;
pub mod auth;

pub struct MockDB {
    pub students: Mutex<HashMap<String, StudentInfo>>,
    pub encrypted: Mutex<HashMap<String, StudentEncrypted>>,
    pub residences: Mutex<HashMap<String, StudentResidence>>,
    pub users: Mutex<HashMap<String, User>>,
    pub tokens: Mutex<HashMap<String, Token>>,
    pub activities: Mutex<HashMap<String, Activity>>,
    pub activity_binds: Mutex<Vec<ActivityBind>>,
}

impl Repository for MockDB {}

impl MockDB {
    pub async fn new() -> Self {
        return Self {
            students:       Mutex::new([].into()),
            encrypted:      Mutex::new([].into()),
            residences:     Mutex::new([].into()),
            users:          Mutex::new([].into()),
            tokens:         Mutex::new([].into()),
            activities:     Mutex::new([].into()),
            activity_binds: Mutex::new([].into()),
        }
    }

    pub async fn init_if_uninitialized(&self) -> Result<(), Error> {
        Ok(())
    }
}