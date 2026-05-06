use axum::async_trait;

use crate::types::Error;
use crate::repository::ActivitiesRepository;
use crate::repository::activities_repository::{Activity, UpdateActivity, SearchActivity, ActivityBind, SearchActivityBind, ActivityBindField};

#[async_trait]
impl ActivitiesRepository for super::MockDB {
    async fn insert_activity(&self, activity: &Activity) -> Result<(), Error> {
        let mut map = self.activities.lock().unwrap();
        map.insert(activity.uuid.clone(), Activity {
            uuid:  activity.uuid.clone(),
            name:  activity.name.clone(),
            staff: activity.staff.clone(),
            dates: activity.dates,
        });
        Ok(())
    }

    async fn update_activity(&self, update: &UpdateActivity) -> Result<(), Error> {
        let mut map = self.activities.lock().unwrap();
        let entry = map.get_mut(&update.uuid)
            .ok_or(Error::InvalidParameter("uuid".to_owned(), update.uuid.clone()))?;

        if let Some(ref name) = update.name {
            entry.name = name.clone();
        }
        if let Some(ref staff) = update.staff {
            let mut arr: [String; 8] = Default::default();
            for (i, s) in staff.iter().enumerate().take(8) {
                arr[i] = s.clone();
            }
            entry.staff = arr;
        }
        if let Some(ref dates) = update.dates {
            let mut arr: [i64; 32] = [0; 32];
            for (i, d) in dates.iter().enumerate().take(32) {
                arr[i] = *d;
            }
            entry.dates = arr;
        }
        Ok(())
    }

    async fn delete_activity(&self, uuid: &str) -> Result<(), Error> {
        let mut map = self.activities.lock().unwrap();
        map.remove(uuid);
        Ok(())
    }

    async fn search_activity(&self, params: &SearchActivity) -> Result<Vec<Activity>, Error> {
        let map = self.activities.lock().unwrap();
        let results: Vec<Activity> = map.values()
            .filter(|a| {
                if let Some(ref name) = params.name {
                    if !a.name.contains(name.as_str()) { return false; }
                }
                if let Some(ref staff) = params.staff {
                    if !a.staff.iter().any(|s| s == staff) { return false; }
                }
                if let Some(date) = params.dates {
                    if !a.dates.iter().any(|d| *d == date) { return false; }
                }
                true
            })
            .map(|a| Activity {
                uuid:  a.uuid.clone(),
                name:  a.name.clone(),
                staff: a.staff.clone(),
                dates: a.dates,
            })
            .collect();
        Ok(results)
    }

    async fn get_activity(&self, uuid: &str) -> Result<Activity, Error> {
        let map = self.activities.lock().unwrap();
        map.get(uuid)
            .map(|a| Activity {
                uuid:  a.uuid.clone(),
                name:  a.name.clone(),
                staff: a.staff.clone(),
                dates: a.dates,
            })
            .ok_or(Error::InvalidParameter("uuid".to_owned(), uuid.to_owned()))
    }

    async fn insert_bind(&self, bind: &ActivityBind) -> Result<(), Error> {
        let mut vec = self.activity_binds.lock().unwrap();
        // Prevent duplicates
        if vec.iter().any(|b| b.student == bind.student && b.activity == bind.activity) {
            return Ok(());
        }
        vec.push(ActivityBind {
            student:  bind.student.clone(),
            activity: bind.activity.clone(),
        });
        Ok(())
    }

    async fn delete_bind(&self, bind: &ActivityBind) -> Result<(), Error> {
        let mut vec = self.activity_binds.lock().unwrap();
        vec.retain(|b| !(b.student == bind.student && b.activity == bind.activity));
        Ok(())
    }

    async fn search_bind(&self, params: &SearchActivityBind) -> Result<Vec<ActivityBind>, Error> {
        let vec = self.activity_binds.lock().unwrap();
        let results: Vec<ActivityBind> = vec.iter()
            .filter(|b| {
                if let Some(ref student) = params.student {
                    if b.student != *student { return false; }
                }
                if let Some(ref activity) = params.activity {
                    if b.activity != *activity { return false; }
                }
                true
            })
            .map(|b| ActivityBind {
                student:  b.student.clone(),
                activity: b.activity.clone(),
            })
            .collect();
        Ok(results)
    }

    async fn delete_all_binds(&self, uuid: &str, field: ActivityBindField) -> Result<(), Error> {
        let mut vec = self.activity_binds.lock().unwrap();
        vec.retain(|b| {
            match field {
                ActivityBindField::Student  => b.student != uuid,
                ActivityBindField::Activity => b.activity != uuid,
            }
        });
        Ok(())
    }
}