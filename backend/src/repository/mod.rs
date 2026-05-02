pub mod student_repository;
pub mod auth_repository;
pub mod activities_repository;

pub use student_repository::StudentRepository;
pub use auth_repository::AuthRepository;
pub use activities_repository::ActivitiesRepository;


pub trait Repository: StudentRepository + AuthRepository + ActivitiesRepository {}