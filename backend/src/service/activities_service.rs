use std::sync::Arc;

use uuid::Uuid;

use crate::endpoints::activities::CreateActivity;
// The service handles the actual logic to doing stuff to the database.
use crate::repository::ActivitiesRepository;

use crate::encryption::{Encryption};

use crate::repository::activities_repository::{Activity, SearchActivity};
use crate::types::Error;

pub struct ActivitiesService {
    repo: Box<dyn ActivitiesRepository>,
    _encryption: Arc<dyn Encryption>
}

impl ActivitiesService {
    pub fn new(repo: Box<dyn ActivitiesRepository>, encryption: Arc<dyn Encryption>) -> Self {
        Self {
            repo: repo,
            _encryption: encryption
        }
    }

    pub async fn create_activity(&self, mut activity: CreateActivity) -> Result<(), Error>{
        let uuid = Uuid::new_v4().to_string();

        if activity.staff.len() < 8 {
            activity.staff.extend(vec![String::from(""); 8-activity.staff.len()]);
        }

        if activity.dates.len() < 32 {
            activity.dates.extend(vec![0; 32-activity.dates.len()]);
        }

        let new_activity = Activity {
            uuid: uuid,
            name: activity.name.clone(),
            staff: activity.staff.try_into().unwrap(),
            dates: activity.dates.try_into().unwrap()
        };

        match self.repo.insert_activity(&new_activity).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Creating Activity".to_owned(), Box::new(t)))
        }
    }

    pub async fn search_activity(&self, params: &SearchActivity) -> Result<Vec<Activity>, Error> {
        match self.repo.search_activity(params).await {
            Ok(t) => Ok(t),
            Err(t) => Err(Error::ErrorDuring("Searching activities".to_owned(), Box::new(t)))
        }
    }
}