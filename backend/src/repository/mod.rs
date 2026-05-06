pub mod student_repository;
pub mod auth_repository;
pub mod activities_repository;

use std::ops::DerefMut;

use axum::async_trait;
use downcast_rs::{DowncastSync, impl_downcast};

pub use student_repository::StudentRepository;
pub use auth_repository::AuthRepository;
pub use activities_repository::ActivitiesRepository;

use crate::database::MockDB;

#[async_trait]
pub trait Repository: 
    StudentRepository + AuthRepository + ActivitiesRepository + 
    DowncastSync {}

impl_downcast!(sync Repository);