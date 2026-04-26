use tokio::sync::Mutex;

use crate::service;

pub mod student;
pub mod admin;
pub mod auth;

pub struct Services {
    pub student: Mutex<service::StudentService>,
    pub admin: Mutex<service::AdminService>,
    pub auth: Mutex<service::AuthService>
}