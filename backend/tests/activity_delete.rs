use std::sync::Arc;

use hsp_backend::{encryption::PlaintextEncryption, repository::{Repository, activities_repository::{Activity, ActivityBind}}, service::ActivitiesService};
use hsp_backend::database::MockDB;

#[tokio::test]
async fn delete_activity_success() {
    let db = MockDB::new().await;
    let enc = PlaintextEncryption::new();
    
    {
        let mut activities = db.activities.lock().unwrap();
        
        activities.insert("activity_uuid".to_owned(), Activity{
            uuid: "activity_uuid".to_owned(),
            name: "Unedited".to_owned(),
            staff: [String::from("Unedited"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [1; 32]
        });

        let mut bind = db.activity_binds.lock().unwrap();
        
        bind.push(ActivityBind {
            student: "Oliver".to_owned(),
            activity: "activity_uuid".to_owned()
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    match service.delete_activity("activity_uuid").await {
        Ok(()) => {},
        Err(t) => panic!("{}", t.to_deobfuscated())
    };

    {
        let downcast = (*arc_db).downcast_ref::<MockDB>().unwrap();
        let activities = downcast.activities.lock().unwrap();
        let activity_binds = downcast.activity_binds.lock().unwrap();
        
        match activities.get(&"activity_uuid".to_owned()) {
            Some(_) => panic!("Activity still exists"),
            None => {}
        };

        match activity_binds.len() {
            0 => {}
            t => panic!("Binds contains {} item{}, should be 0", t, if t == 1 {""} else {"s"}),
        };
        
    }
}

#[tokio::test]
async fn delete_activity_fail() {
    let db = MockDB::new().await;
    let enc = PlaintextEncryption::new();
    
    {
        let mut activities = db.activities.lock().unwrap();
        
        activities.insert("activity_uuid2".to_owned(), Activity{
            uuid: "activity_uuid2".to_owned(),
            name: "Unedited".to_owned(),
            staff: [String::from("Unedited"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [1; 32]
        });

        let mut bind = db.activity_binds.lock().unwrap();
        
        bind.push(ActivityBind {
            student: "Mark".to_owned(),
            activity: "activity_uuid2".to_owned()
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    match service.delete_activity("activity_uuid").await {
        Ok(()) => {},
        Err(t) => panic!("{}", t.to_deobfuscated())
    };

    {
        let downcast = (*arc_db).downcast_ref::<MockDB>().unwrap();
        let activities = downcast.activities.lock().unwrap();
        let activity_binds = downcast.activity_binds.lock().unwrap();
        
        match activities.get(&"activity_uuid2".to_owned()) {
            Some(_) => {},
            None => panic!("Activity does not exist")
        };

        match activity_binds.len() {
            1 => {}
            t => panic!("Binds contains {} items, should be 0", t),
        };
        
    }
}