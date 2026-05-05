pub mod student_repository;
pub mod auth_repository;
pub mod activities_repository;

use axum::async_trait;
pub use student_repository::StudentRepository;
pub use auth_repository::AuthRepository;
pub use activities_repository::ActivitiesRepository;

use crate::types::Error;

#[async_trait]
pub trait Repository: StudentRepository + AuthRepository + ActivitiesRepository {}