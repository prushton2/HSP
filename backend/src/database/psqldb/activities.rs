use crate::repository::ActivitiesRepository;
use crate::repository::activities_repository::{Activity, UpdateActivity, SearchActivity, ActivityBind, SearchActivityBind, ActivityBindField};

use crate::types::Error;

use axum::async_trait;
use tokio_postgres::types::ToSql;

#[async_trait]
impl ActivitiesRepository for super::PSQLDB {
    async fn insert_activity(&self, activity: &Activity) -> Result<(), Error> {
        let staff: Vec<&str> = activity.staff.iter().map(|s| s.as_str()).collect();
        let dates: Vec<i64> = activity.dates.to_vec();
        return match self.client.execute(
            "INSERT INTO Activities (uuid, name, staff, dates) VALUES ($1, $2, $3, $4)",
            &[&activity.uuid, &activity.name, &staff, &dates]
        ).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Inserting activity".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn update_activity(&self, uuid: &str, update: &UpdateActivity) -> Result<(), Error> {
        let mut set_clauses: Vec<String> = Vec::new();
        let mut params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
        let mut idx = 1;

        if let Some(name) = &update.name {
            set_clauses.push(format!("name = ${}", idx));
            params.push(Box::new(name.clone()));
            idx += 1;
        }
        if let Some(staff) = &update.staff {
            set_clauses.push(format!("staff = ${}", idx));
            params.push(Box::new(staff.clone()));
            idx += 1;
        }
        if let Some(dates) = &update.dates {
            set_clauses.push(format!("dates = ${}", idx));
            params.push(Box::new(dates.clone()));
            idx += 1;
        }

        if set_clauses.is_empty() {
            return Ok(());
        }

        params.push(Box::new(uuid.to_owned()));
        let query = format!("UPDATE Activities SET {} WHERE uuid = ${}", set_clauses.join(", "), idx);

        let param_refs: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref() as &(dyn ToSql + Sync)).collect();

        return match self.client.execute(&query, &param_refs[..]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Updating activity".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn delete_activity(&self, uuid: &str) -> Result<(), Error> {
        return match self.client.execute("DELETE FROM Activities WHERE uuid = $1", &[&uuid]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Deleting activity".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn search_activity(&self, params: &SearchActivity) -> Result<Vec<Activity>, Error> {
        let mut clauses: Vec<String> = Vec::new();
        let mut query_params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
        let mut idx = 1;

        if let Some(name) = &params.name {
            clauses.push(format!("name = ${}", idx));
            query_params.push(Box::new(name.clone()));
            idx += 1;
        }
        if let Some(staff) = &params.staff {
            clauses.push(format!("${} = ANY(staff)", idx));
            query_params.push(Box::new(staff.clone()));
            idx += 1;
        }
        if let Some(dates) = &params.dates {
            clauses.push(format!("${} = ANY(dates)", idx));
            query_params.push(Box::new(*dates));
        }

        let param_refs: Vec<&(dyn ToSql + Sync)> = query_params.iter().map(|p| p.as_ref() as &(dyn ToSql + Sync)).collect();

        let query_string = format!("SELECT * FROM Activities {} {}",
            if clauses.len() != 0 { "WHERE" } else { "" },
            clauses.join(" AND ")
        );

        let rows = match self.client.query(&query_string, &param_refs[..]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Searching activities".to_owned(), Box::new(Error::PostgresError(t))))
        };

        let mut vec: Vec<Activity> = vec![];
        for row in rows {
            let staff_vec: Vec<String> = row.get::<&str, Vec<String>>("staff");
            let dates_vec: Vec<i64> = row.get::<&str, Vec<i64>>("dates");

            let mut staff_arr: [String; 8] = Default::default();
            for (i, s) in staff_vec.iter().enumerate().take(8) {
                staff_arr[i] = s.clone();
            }

            let mut dates_arr: [i64; 32] = [0; 32];
            for (i, d) in dates_vec.iter().enumerate().take(32) {
                dates_arr[i] = *d;
            }

            vec.push(Activity {
                uuid:  row.get::<&str, &str>("uuid").to_string(),
                name:  row.get::<&str, &str>("name").to_string(),
                staff: staff_arr,
                dates: dates_arr,
            });
        }

        Ok(vec)
    }

    async fn get_activity(&self, uuid: &str) -> Result<Activity, Error> {
        let row = match self.client.query_one("SELECT * FROM Activities WHERE uuid = $1", &[&uuid]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Getting activity".to_owned(), Box::new(Error::PostgresError(t))))
        };

        let staff_vec: Vec<String> = row.get::<&str, Vec<String>>("staff");
        let dates_vec: Vec<i64> = row.get::<&str, Vec<i64>>("dates");

        let mut staff_arr: [String; 8] = Default::default();
        for (i, s) in staff_vec.iter().enumerate().take(8) {
            staff_arr[i] = s.clone();
        }

        let mut dates_arr: [i64; 32] = [0; 32];
        for (i, d) in dates_vec.iter().enumerate().take(32) {
            dates_arr[i] = *d;
        }

        Ok(Activity {
            uuid:  row.get::<&str, &str>("uuid").to_string(),
            name:  row.get::<&str, &str>("name").to_string(),
            staff: staff_arr,
            dates: dates_arr,
        })
    }

    async fn insert_bind(&self, bind: &ActivityBind) -> Result<(), Error> {
        return match self.client.execute(
            "INSERT INTO StudentActivities (Student, Activity) VALUES ($1, $2)",
            &[&bind.student, &bind.activity]
        ).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Inserting activity bind".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn delete_bind(&self, bind: &ActivityBind) -> Result<(), Error> {
        return match self.client.execute(
            "DELETE FROM StudentActivities WHERE Student = $1 AND Activity = $2",
            &[&bind.student, &bind.activity]
        ).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Deleting activity bind".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }

    async fn search_bind(&self, params: &SearchActivityBind) -> Result<Vec<ActivityBind>, Error> {
        let mut clauses: Vec<String> = Vec::new();
        let mut query_params: Vec<Box<dyn ToSql + Sync + Send>> = Vec::new();
        let mut idx = 1;

        if let Some(student) = &params.student {
            clauses.push(format!("Student = ${}", idx));
            query_params.push(Box::new(student.clone()));
            idx += 1;
        }
        if let Some(activity) = &params.activity {
            clauses.push(format!("Activity = ${}", idx));
            query_params.push(Box::new(activity.clone()));
        }

        let param_refs: Vec<&(dyn ToSql + Sync)> = query_params.iter().map(|p| p.as_ref() as &(dyn ToSql + Sync)).collect();

        let query_string = format!("SELECT * FROM StudentActivities {} {}",
            if clauses.len() != 0 { "WHERE" } else { "" },
            clauses.join(" AND ")
        );

        let rows = match self.client.query(&query_string, &param_refs[..]).await {
            Ok(t) => t,
            Err(t) => return Err(Error::ErrorDuring("Searching activity binds".to_owned(), Box::new(Error::PostgresError(t))))
        };

        let mut vec: Vec<ActivityBind> = vec![];
        for row in rows {
            vec.push(ActivityBind {
                student:  row.get::<&str, &str>("student").to_string(),
                activity: row.get::<&str, &str>("activity").to_string(),
            });
        }

        Ok(vec)
    }

    async fn delete_all_binds(&self, uuid: &str, field: ActivityBindField) -> Result<(), Error> {
        let query = match field {
            ActivityBindField::Student  => "DELETE FROM StudentActivities WHERE Student = $1",
            ActivityBindField::Activity => "DELETE FROM StudentActivities WHERE Activity = $1",
        };

        return match self.client.execute(query, &[&uuid]).await {
            Ok(_) => Ok(()),
            Err(t) => Err(Error::ErrorDuring("Deleting all activity binds".to_owned(), Box::new(Error::PostgresError(t))))
        };
    }
}