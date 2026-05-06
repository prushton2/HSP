pub mod database;
pub mod endpoints;
pub mod encryption;
pub mod repository;
pub mod service;
pub mod types;

pub const TOKEN_EXPIRY: i64 = 3024000;
pub const SIGNUP_HASH_EXPIRY: i64 = 86400;