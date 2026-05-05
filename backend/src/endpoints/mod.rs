use crate::service;

pub mod student;
pub mod admin;
pub mod auth;
pub mod activities;

pub struct Services {
    pub student: service::StudentService,
    pub admin: service::AdminService,
    pub auth: service::AuthService,
    pub activities: service::ActivitiesService
}