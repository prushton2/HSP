use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::database;
