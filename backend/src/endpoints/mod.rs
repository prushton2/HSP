use tokio::sync::RwLock;

use crate::service;

pub mod student;
pub mod admin;
pub mod auth;
pub mod activities;

pub struct Services {
    pub student: RwLock<service::StudentService>,
    pub admin: RwLock<service::AdminService>,
    pub auth: RwLock<service::AuthService>,
    pub activities: RwLock<service::ActivitiesService>
}