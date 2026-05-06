use std::sync::Arc;

    use hsp_backend::{encryption::PlaintextEncryption, repository::{Repository, activities_repository::{Activity, SearchActivity}}, service::ActivitiesService};
use hsp_backend::database::MockDB;

async fn init_repo_enc() -> (Arc<dyn Repository>, PlaintextEncryption) {
    return (Arc::new(MockDB::new().await), PlaintextEncryption::new())
}


#[tokio::test]
async fn get_activity_none() {
    let (db, enc) = init_repo_enc().await;
    let service = ActivitiesService::new(Arc::clone(&db), Arc::new(enc));
    
    match service.get_activity("fake", false, false).await {
        Ok(_) => {panic!("Returned OK when no activity could be found")},
        Err(t) => t
    };
}

#[tokio::test]
async fn search_activity_by_name_find_none() {
    let db = MockDB::new().await;
    let enc = PlaintextEncryption::new();
    
    {
        let mut activities = db.activities.lock().unwrap();
        
        activities.insert("activity_uuid".to_owned(), Activity{
            uuid: "activity_uuid".to_owned(),
            name: "Activity".to_owned(),
            staff: [String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [0; 32]
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    let vec = match service.search_activity(&SearchActivity {
        name: Some("Bad Name".to_owned()),
        staff: None,
        dates: None
    }).await {
        Ok(t) => t,
        Err(t) => {panic!("{}", t.to_deobfuscated())},
    };

    assert!(vec.len() == 0);
}

#[tokio::test]
async fn search_activity_by_name_find_some() {
    let db = MockDB::new().await;
    let enc = PlaintextEncryption::new();
    
    {
        let mut activities = db.activities.lock().unwrap();
        
        activities.insert("activity_uuid".to_owned(), Activity{
            uuid: "activity_uuid".to_owned(),
            name: "Activity".to_owned(),
            staff: [String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [0; 32]
        });

        activities.insert("activity_uuid2".to_owned(), Activity{
            uuid: "activity_uuid2".to_owned(),
            name: "Activity Name".to_owned(),
            staff: [String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [0; 32]
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    let vec = match service.search_activity(&SearchActivity {
        name: Some("Activity".to_owned()),
        staff: None,
        dates: None
    }).await {
        Ok(t) => t,
        Err(t) => {panic!("{}", t.to_deobfuscated())},
    };

    assert!(vec.len() == 2);
}


#[tokio::test]
async fn search_activity_by_staff_find_none() {
    let db = MockDB::new().await;
    let enc = PlaintextEncryption::new();
    
    {
        let mut activities = db.activities.lock().unwrap();
        
        activities.insert("activity_uuid".to_owned(), Activity{
            uuid: "activity_uuid".to_owned(),
            name: "Activity".to_owned(),
            staff: [String::from("Ryland Grace"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [0; 32]
        });

        activities.insert("activity_uuid2".to_owned(), Activity{
            uuid: "activity_uuid2".to_owned(),
            name: "Activity Name".to_owned(),
            staff: [String::from("Allen Turing"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [0; 32]
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    let vec = match service.search_activity(&SearchActivity {
        name: None,
        staff: Some("Nolan".to_owned()),
        dates: None
    }).await {
        Ok(t) => t,
        Err(t) => {panic!("{}", t.to_deobfuscated())},
    };

    assert!(vec.len() == 0);
}

#[tokio::test]
async fn search_activity_by_staff_find_some() {
    let db = MockDB::new().await;
    let enc = PlaintextEncryption::new();
    
    {
        let mut activities = db.activities.lock().unwrap();
        
        activities.insert("activity_uuid".to_owned(), Activity{
            uuid: "activity_uuid".to_owned(),
            name: "Activity".to_owned(),
            staff: [String::from("Ryland Grace"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [0; 32]
        });

        activities.insert("activity_uuid2".to_owned(), Activity{
            uuid: "activity_uuid2".to_owned(),
            name: "Activity Name".to_owned(),
            staff: [String::from("Allen Turing"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [0; 32]
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    let vec = match service.search_activity(&SearchActivity {
        name: None,
        staff: Some("Ryland Grace".to_owned()),
        dates: None
    }).await {
        Ok(t) => t,
        Err(t) => {panic!("{}", t.to_deobfuscated())},
    };

    assert_eq!(vec.len(), 1);
}


#[tokio::test]
async fn search_activity_by_date_find_none() {
    let db = MockDB::new().await;
    let enc = PlaintextEncryption::new();
    
    {
        let mut activities = db.activities.lock().unwrap();
        
        activities.insert("activity_uuid".to_owned(), Activity{
            uuid: "activity_uuid".to_owned(),
            name: "Activity".to_owned(),
            staff: [String::from("Ryland Grace"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [3; 32]
        });

        activities.insert("activity_uuid2".to_owned(), Activity{
            uuid: "activity_uuid2".to_owned(),
            name: "Activity Name".to_owned(),
            staff: [String::from("Allen Turing"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [12; 32]
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    let vec = match service.search_activity(&SearchActivity {
        name: None,
        staff: None,
        dates: Some(2)
    }).await {
        Ok(t) => t,
        Err(t) => {panic!("{}", t.to_deobfuscated())},
    };

    assert!(vec.len() == 0);
}

#[tokio::test]
async fn search_activity_by_date_find_some() {
    let db = MockDB::new().await;
    let enc = PlaintextEncryption::new();
    
    {
        let mut activities = db.activities.lock().unwrap();
        
        activities.insert("activity_uuid".to_owned(), Activity{
            uuid: "activity_uuid".to_owned(),
            name: "Activity".to_owned(),
            staff: [String::from("Ryland Grace"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [3; 32]
        });

        activities.insert("activity_uuid2".to_owned(), Activity{
            uuid: "activity_uuid2".to_owned(),
            name: "Activity Name".to_owned(),
            staff: [String::from("Allen Turing"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [12; 32]
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    let vec = match service.search_activity(&SearchActivity {
        name: None,
        staff: None,
        dates: Some(12)
    }).await {
        Ok(t) => t,
        Err(t) => {panic!("{}", t.to_deobfuscated())},
    };

    assert_eq!(vec.len(), 1);
}

#[tokio::test]
async fn search_activity_by_multiple_find_none() {
    let db = MockDB::new().await;
    let enc = PlaintextEncryption::new();
    
    {
        let mut activities = db.activities.lock().unwrap();
        
        activities.insert("activity_uuid".to_owned(), Activity{
            uuid: "activity_uuid".to_owned(),
            name: "Activity".to_owned(),
            staff: [String::from("Ryland Grace"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [3; 32]
        });

        activities.insert("activity_uuid2".to_owned(), Activity{
            uuid: "activity_uuid2".to_owned(),
            name: "Activity Name".to_owned(),
            staff: [String::from("Allen Turing"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [12; 32]
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    let vec = match service.search_activity(&SearchActivity {
        name: Some("Lucan".to_owned()),
        staff: None,
        dates: Some(12)
    }).await {
        Ok(t) => t,
        Err(t) => {panic!("{}", t.to_deobfuscated())},
    };

    assert!(vec.len() == 0);
}

#[tokio::test]
async fn search_activity_by_multiple_find_some() {
    let db = MockDB::new().await;
    let enc = PlaintextEncryption::new();
    
    {
        let mut activities = db.activities.lock().unwrap();
        
        activities.insert("activity_uuid".to_owned(), Activity{
            uuid: "activity_uuid".to_owned(),
            name: "Activity".to_owned(),
            staff: [String::from("Ryland Grace"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [3; 32]
        });

        activities.insert("activity_uuid2".to_owned(), Activity{
            uuid: "activity_uuid2".to_owned(),
            name: "Activity Name".to_owned(),
            staff: [String::from("Allen Turing"), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from(""), String::from("")],
            dates: [12; 32]
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    let vec = match service.search_activity(&SearchActivity {
        name: Some("Activity".to_owned()),
        staff: None,
        dates: Some(12)
    }).await {
        Ok(t) => t,
        Err(t) => {panic!("{}", t.to_deobfuscated())},
    };

    assert_eq!(vec.len(), 1);
}