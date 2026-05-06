use std::sync::Arc;

use hsp_backend::{encryption::{PlaintextEncryption}, endpoints::activities::CreateActivity, repository::Repository, service::ActivitiesService};
use hsp_backend::database::MockDB;
use hsp_backend::types::Error;

async fn init_repo_enc() -> (Arc<dyn Repository>, PlaintextEncryption) {
    return (Arc::new(MockDB::new().await), PlaintextEncryption::new())
}


#[tokio::test]
async fn create_activity_success() {
    let (db, enc) = init_repo_enc().await;
    let service = ActivitiesService::new(Arc::clone(&db), Arc::new(enc));
    
    match service.create_activity(CreateActivity {
        name: String::from("Activity"),
        staff: vec![String::from("1")],
        dates: vec![13]
    }).await {
        Ok(_) => {},
        Err(t) => panic!("{}", t.to_deobfuscated())
    }

    let mockdb = (*db).as_any().downcast_ref::<MockDB>().unwrap();
    let lock = mockdb.activities.lock().unwrap();

    for (_, activity) in lock.iter() {
        assert!(activity.name == "Activity");
        assert!(activity.staff.len() == 8);  // Padding fails
        assert!(activity.staff[0] == "1");   // Copying into db fails
        assert!(activity.dates.len() == 32); // Padding fails
        assert!(activity.dates[0] == 13);    // Copying into db fails
    }
}

#[tokio::test]
async fn create_activity_too_many_staff() {
    let (db, enc) = init_repo_enc().await;
    let service = ActivitiesService::new(Arc::clone(&db), Arc::new(enc));
    
    let error = match service.create_activity(CreateActivity {
        name: String::from("Activity"),
        staff: vec![String::from("1"), String::from("1"), String::from("1"), String::from("1"), String::from("1"), String::from("1"), String::from("1"), String::from("1"), String::from("1")],
        dates: vec![13]
    }).await {
        Ok(_) => panic!("Expected error (too many staff provided)"),
        Err(t) => t
    };

    match error {
        Error::InvalidParameter(_, _) => {},
        t => panic!("Expected invalid parameter error, got {}", t.to_deobfuscated())
    }
}

#[tokio::test]
async fn create_activity_too_many_dates() {
    let (db, enc) = init_repo_enc().await;
    let service = ActivitiesService::new(Arc::clone(&db), Arc::new(enc));
    
    let error = match service.create_activity(CreateActivity {
        name: String::from("Activity"),
        staff: vec![String::from("1")],
        dates: vec![13; 33]
    }).await {
        Ok(_) => panic!("Expected error (too many staff provided)"),
        Err(t) => t
    };

    match error {
        Error::InvalidParameter(_, _) => {},
        t => panic!("Expected invalid parameter error, got {}", t.to_deobfuscated())
    }
}

#[tokio::test]
async fn create_activity_too_many_dates_and_staff() {
    let (db, enc) = init_repo_enc().await;
    let service = ActivitiesService::new(Arc::clone(&db), Arc::new(enc));
    
    let error = match service.create_activity(CreateActivity {
        name: String::from("Activity"),
        staff: vec![String::from("1"); 21],
        dates: vec![13; 33]
    }).await {
        Ok(_) => panic!("Expected error (too many staff provided)"),
        Err(t) => t
    };

    match error {
        Error::InvalidParameter(_, _) => {},
        t => panic!("Expected invalid parameter error, got {}", t.to_deobfuscated())
    }
}