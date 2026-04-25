use tokio::sync::Mutex;

use crate::service;

// pub mod auth;
pub mod admin;

pub struct Services {
    pub student: Mutex<service::StudentService>,
    pub admin: Mutex<service::AdminService>
}