use crate::repository::ActivitiesRepository;
use crate::repository::activities_repository::{Activity, UpdateActivity, SearchActivity, ActivityBind, SearchActivityBind, ActivityBindField};

use crate::types::Error;

use axum::async_trait;
use tokio_postgres::types::ToSql;

#[async_trait]
impl ActivitiesRepository for super::MockDB {
    async fn insert_activity(&self, activity: &Activity) -> Result<(), Error> {
    }

    async fn update_activity(&self, update: &UpdateActivity) -> Result<(), Error> {
    }

    async fn delete_activity(&self, uuid: &str) -> Result<(), Error> {
    }

    async fn search_activity(&self, params: &SearchActivity) -> Result<Vec<Activity>, Error> {
    }

    async fn get_activity(&self, uuid: &str) -> Result<Activity, Error> {
    }

    async fn insert_bind(&self, bind: &ActivityBind) -> Result<(), Error> {
    }

    async fn delete_bind(&self, bind: &ActivityBind) -> Result<(), Error> {
    }

    async fn search_bind(&self, params: &SearchActivityBind) -> Result<Vec<ActivityBind>, Error> {
    }

    async fn delete_all_binds(&self, uuid: &str, field: ActivityBindField) -> Result<(), Error> {
    }
}