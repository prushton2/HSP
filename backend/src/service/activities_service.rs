// The service handles the actual logic to doing stuff to the database.
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use uuid::Uuid;

use crate::endpoints::activities::CreateActivity;
use crate::repository::{Repository, activities_repository::ActivityBindField};
use crate::encryption::{Encryption};

use crate::repository::activities_repository::{Activity, ActivityBind, SearchActivity, SearchActivityBind, UpdateActivity};
use crate::repository::student_repository::{SearchStudentResidence, SearchStudentInfo};
use crate::service::student_service::FullStudent;
use crate::types::Error;

pub struct ActivitiesService {
    repo: Box<dyn Repository>,
    encryption: Arc<dyn Encryption>
}

impl ActivitiesService {
    pub fn new(repo: Box<dyn Repository>, encryption: Arc<dyn Encryption>) -> Self {
        Self {
            repo: repo,
            encryption: encryption
        }
    }

    pub async fn create_activity(&self, mut activity: CreateActivity) -> Result<(), Error>{
        let uuid = Uuid::new_v4().to_string();

        if activity.staff.len() < 8 {
            activity.staff.extend(vec![String::from(""); 8-activity.staff.len()]);
        } else if activity.staff.len() > 8 {
            return Err(Error::InvalidParameter("staff".to_owned(), "Count cannot exceed 8".to_owned()))
        }

        if activity.dates.len() < 32 {
            activity.dates.extend(vec![0; 32-activity.dates.len()]);
        } else if activity.dates.len() > 32 {
            return Err(Error::InvalidParameter("dates".to_owned(), "Count cannot exceed 32".to_owned()))
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

    pub async fn edit_activity(&self, mut update: UpdateActivity) -> Result<(), Error>{

        if let Some(staff) = &mut update.staff {
            if staff.len() < 8 {
                let len = staff.len();
                staff.extend(vec![String::from(""); 8-len]);
            } else if staff.len() > 8 {
                return Err(Error::InvalidParameter("staff".to_owned(), "Count cannot exceed 8".to_owned()))
            }
        }

        if let Some(dates) = &mut update.dates {
            if dates.len() < 32 {
                let len = dates.len();
                dates.extend(vec![0; 8-len]);
            } else if dates.len() > 32 {
                return Err(Error::InvalidParameter("dates".to_owned(), "Count cannot exceed 32".to_owned()))
            }
        }

        match self.repo.update_activity(&update).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Editing Activity".to_owned(), Box::new(t)))
        }
    }

    pub async fn search_activity(&self, params: &SearchActivity) -> Result<Vec<Activity>, Error> {
        match self.repo.search_activity(params).await {
            Ok(t) => Ok(t),
            Err(t) => Err(Error::ErrorDuring("Searching activities".to_owned(), Box::new(t)))
        }
    }

    pub async fn delete_activity(&self, uuid: &str) -> Result<(), Error> {
        match self.repo.delete_activity(uuid).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Deleting activities".to_owned(), Box::new(t)))
        }

        match self.repo.delete_all_binds(uuid, ActivityBindField::Activity).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Deleting binds".to_owned(), Box::new(t)))
        }

        Ok(())
    }

    pub async fn get_activity(&self, uuid: &str, get_attendees: bool, decrypt: bool) -> Result<(Activity, Vec<FullStudent>), Error> {
        let activity = match self.repo.get_activity(uuid).await {
            Ok(t) => t,
            Err(t) =>  return Err(Error::ErrorDuring("Getting Activity".to_owned(), Box::new(t)))
        };

        if !get_attendees {
            return Ok((activity, vec![]));
        }

        let attendee_uuids = match self.repo.search_bind(&SearchActivityBind{student: None, activity: Some(uuid.to_owned())}).await {
            Ok(t) => t.iter().map(|f| f.student.clone()).collect::<HashSet<String>>(),
            Err(t) => return Err(Error::ErrorDuring("Getting attendee uuids".to_owned(), Box::new(t)))
        };

        
        let mut attendee_info = match self.repo.search_studentinfo(&SearchStudentInfo{uuid: "".to_owned(), fname: None, lname: None, number: None}).await {
            Ok(t) => {
                t.iter()
                    .filter(|f| attendee_uuids.contains(&f.uuid))
                    .map(|f| (f.uuid.to_owned(), 
                        FullStudent{fname: "".to_owned(), lname: "".to_owned(), pronouns: "".to_owned(), number: f.number, hall: "".to_owned(), room: 0, wing: "".to_owned()}))
                    .collect::<HashMap<String, FullStudent>>()
            },
            Err(t) => return Err(Error::ErrorDuring("Getting attendee info".to_owned(), Box::new(t)))
        };

        match self.repo.search_residence(&SearchStudentResidence{uuid: "".to_owned(), hall: None, room: None, wing: None}).await {
            Ok(t) => {
                for info in t {
                    match attendee_info.get_mut(&info.uuid) {
                        Some(t) => {
                            t.room = info.room;
                            t.wing = info.wing;
                            t.hall = info.hall;
                        },
                        None => {}
                    };
                }
            }
            Err(t) => return Err(Error::ErrorDuring("Getting residence info".to_owned(), Box::new(t)))
        }

        if !decrypt {
            return Ok((activity, attendee_info.into_values().collect()))
        }

        match self.repo.getall_encrypted().await {
            Ok(t) => {
                for info in t {
                    match attendee_info.get_mut(&info.uuid) {
                        Some(t) => {
                            let decrypted = self.encryption.decrypt(&info.data);

                            t.fname    = decrypted.first_name;
                            t.lname    = decrypted.last_name;
                            t.pronouns = decrypted.pronouns;
                        },
                        None => {}
                    };
                }
            },
            Err(t) => return Err(Error::ErrorDuring("Getting decrypyed data".to_owned(), Box::new(t)))
        }

        Ok((activity, attendee_info.into_values().collect()))
    }

    pub async fn bind_students(&self, uuid: &str, students: Vec<i32>) -> Result<(), Error> {

        let student_set: HashSet<i32> = students.into_iter().collect();

        match self.repo.delete_all_binds(uuid, ActivityBindField::Activity).await {
            Ok(_) => {},
            Err(t) => return Err(Error::ErrorDuring("Deleting existing binds".to_owned(), Box::new(t)))
        }

        let student_uuids = match self.repo.search_studentinfo(&SearchStudentInfo{uuid: "".to_owned(), fname: None, lname: None, number: None}).await {
            Ok(t) => {
                t.iter()
                    .filter(|f| student_set.contains(&f.number))
                    .map(|f| f.uuid.clone())
                    .collect::<Vec<String>>()
            },
            Err(t) => return Err(Error::ErrorDuring("Getting attendee info".to_owned(), Box::new(t)))
        };

        for student in student_uuids.into_iter() {
            match self.repo.insert_bind(&ActivityBind{activity: uuid.to_owned(), student: student}).await {
                Ok(_) => {},
                Err(t) => return Err(Error::ErrorDuring(format!("Binding to activity {}", uuid), Box::new(t)))   
            }
        }

        Ok(())
    }
}