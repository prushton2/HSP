// this holds the traits that directly interface with the database. These can be easily faked for tests.
use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::types::Error;

#[async_trait]
pub trait ActivitiesRepository: Send + Sync {
    async fn insert_activity(&self, activity: &Activity) -> Result<(), Error>;
    async fn update_activity(&self, uuid: &str, update: &UpdateActivity) -> Result<(), Error>;
    async fn delete_activity(&self, uuid: &str) -> Result<(), Error>;
    async fn search_activity(&self, params: &SearchActivity) -> Result<Vec<Activity>, Error>;
    async fn get_activity   (&self, uuid: &str) -> Result<Activity, Error>;

    async fn insert_bind(&self, bind: &ActivityBind) -> Result<(), Error>;
    async fn delete_bind(&self, bind: &ActivityBind) -> Result<(), Error>;    
    async fn search_bind(&self, params: &SearchActivityBind) -> Result<Vec<ActivityBind>, Error>;
    async fn delete_all_binds(&self, uuid: &str, field: ActivityBindField) -> Result<(), Error>;
}

#[derive(Serialize, Deserialize)]
pub struct Activity {
    pub uuid:  String,
    pub name:  String,
    pub staff: [String; 8],
    pub dates: [i64; 32]
}
#[derive(Serialize, Deserialize)]
pub struct UpdateActivity {
    pub uuid:  String,
    pub name:  Option<String>,
    pub staff: Option<Vec<String>>,
    pub dates: Option<Vec<i64>>
}
#[derive(Serialize, Deserialize)]
pub struct SearchActivity {
    pub name:  Option<String>,
    pub staff: Option<String>,
    pub dates: Option<i64>
}
#[derive(Serialize, Deserialize)]
pub struct ActivityBind {
    pub student: String,
    pub activity: String
}
#[derive(Serialize, Deserialize)]
pub struct SearchActivityBind {
    pub student: Option<String>,
    pub activity: Option<String>
}

pub type DeleteActivityBinds = SearchActivityBind;

#[derive(Copy, Clone)]
pub enum ActivityBindField {
    Student,
    Activity
}