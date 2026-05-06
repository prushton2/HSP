use std::sync::Arc;

use hsp_backend::{encryption::PlaintextEncryption, repository::{Repository, activities_repository::{Activity, UpdateActivity}}, service::ActivitiesService};
use hsp_backend::database::MockDB;

#[tokio::test]
async fn edit_activity_none() {
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
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    match service.edit_activity(UpdateActivity {
        uuid: "activity_uuid".to_owned(),
        name: None,
        staff: None,
        dates: None
    }).await {
        Ok(()) => {},
        Err(t) => panic!("{}", t.to_deobfuscated())
    };

    {
        let downcast = (*arc_db).downcast_ref::<MockDB>().unwrap();
        let activities = downcast.activities.lock().unwrap();
        let edited = match activities.get(&"activity_uuid".to_owned()) {
            Some(t) => t,
            None => panic!("Activity could not be found")
        };

        assert_eq!(edited.name, "Unedited");
        assert_eq!(edited.staff[0], "Unedited");
        assert_eq!(edited.dates[0], 1);
        
    }
}

#[tokio::test]
async fn edit_activity_name() {
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
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    match service.edit_activity(UpdateActivity {
        uuid: "activity_uuid".to_owned(),
        name: Some("Edited".to_owned()),
        staff: None,
        dates: None
    }).await {
        Ok(()) => {},
        Err(t) => panic!("{}", t.to_deobfuscated())
    };

    {
        let downcast = (*arc_db).downcast_ref::<MockDB>().unwrap();
        let activities = downcast.activities.lock().unwrap();
        let edited = match activities.get(&"activity_uuid".to_owned()) {
            Some(t) => t,
            None => panic!("Activity could not be found")
        };

        assert_eq!(edited.name, "Edited");
        assert_eq!(edited.staff[0], "Unedited");
        assert_eq!(edited.dates[0], 1);
        
    }
}

#[tokio::test]
async fn edit_activity_staff() {
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
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    match service.edit_activity(UpdateActivity {
        uuid: "activity_uuid".to_owned(),
        name: None,
        staff: Some(vec!["Edited".to_string()]),
        dates: None
    }).await {
        Ok(()) => {},
        Err(t) => panic!("{}", t.to_deobfuscated())
    };

    {
        let downcast = (*arc_db).downcast_ref::<MockDB>().unwrap();
        let activities = downcast.activities.lock().unwrap();
        let edited = match activities.get(&"activity_uuid".to_owned()) {
            Some(t) => t,
            None => panic!("Activity could not be found")
        };

        assert_eq!(edited.name, "Unedited");
        assert_eq!(edited.staff[0], "Edited");
        assert_eq!(edited.dates[0], 1);
        
    }
}

#[tokio::test]
async fn edit_activity_dates() {
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
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    match service.edit_activity(UpdateActivity {
        uuid: "activity_uuid".to_owned(),
        name: None,
        staff: None,
        dates: Some(vec![2])
    }).await {
        Ok(()) => {},
        Err(t) => panic!("{}", t.to_deobfuscated())
    };

    {
        let downcast = (*arc_db).downcast_ref::<MockDB>().unwrap();
        let activities = downcast.activities.lock().unwrap();
        let edited = match activities.get(&"activity_uuid".to_owned()) {
            Some(t) => t,
            None => panic!("Activity could not be found")
        };

        assert_eq!(edited.name, "Unedited");
        assert_eq!(edited.staff[0], "Unedited");
        assert_eq!(edited.dates[0], 2);
        
    }
}

#[tokio::test]
async fn edit_activity_all() {
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
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    match service.edit_activity(UpdateActivity {
        uuid: "activity_uuid".to_owned(),
        name:  Some("Edited".to_owned()),
        staff: Some(vec!["Edited".to_owned()]),
        dates: Some(vec![2])
    }).await {
        Ok(()) => {},
        Err(t) => panic!("{}", t.to_deobfuscated())
    };

    {
        let downcast = (*arc_db).downcast_ref::<MockDB>().unwrap();
        let activities = downcast.activities.lock().unwrap();
        let edited = match activities.get(&"activity_uuid".to_owned()) {
            Some(t) => t,
            None => panic!("Activity could not be found")
        };

        assert_eq!(edited.name, "Edited");
        assert_eq!(edited.staff[0], "Edited");
        assert_eq!(edited.dates[0], 2);
        
    }
}