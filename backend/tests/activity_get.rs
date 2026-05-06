use std::sync::Arc;

use hsp_backend::{encryption::PlaintextEncryption, repository::{Repository, activities_repository::{Activity, ActivityBind}, student_repository::{StudentEncrypted, StudentInfo, StudentResidence}}, service::ActivitiesService};
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
async fn get_activity_activity() {
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
    
    let (activity, students) =match service.get_activity("activity_uuid", false, false).await {
        Ok(t) => t,
        Err(t) => {panic!("No activity found: {}", t.to_deobfuscated())},
    };

    assert!(activity.name == "Activity");
    assert!(students.len() == 0);
}

#[tokio::test]
async fn get_activity_activity_students() {
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

        let mut students = db.students.lock().unwrap();

        students.insert("student_uuid".to_owned(), StudentInfo {
            uuid: "student_uuid".to_owned(),
            number: 3,
            fname: String::from(""),
            lname: String::from("")
        });

        let mut residence = db.residences.lock().unwrap();

        residence.insert("student_uuid".to_owned(), StudentResidence {
            uuid: "student_uuid".to_owned(),
            hall: "Shecon".to_owned(),
            room: 212,
            wing: "Mario".to_owned()
        });

        let mut binding = db.activity_binds.lock().unwrap();

        binding.push(ActivityBind{
            student: "student_uuid".to_owned(),
            activity: "activity_uuid".to_owned()
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    let (activity, students) =match service.get_activity("activity_uuid", true, false).await {
        Ok(t) => t,
        Err(t) => {panic!("No activity found: {}", t.to_deobfuscated())},
    };

    assert!(activity.name == "Activity");
    assert!(students.len() == 1);
    assert!(students[0].number == 3);
    assert!(students[0].fname == "");
}

#[tokio::test]
async fn get_activity_activity_students_encrypted() {
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

        let mut students = db.students.lock().unwrap();

        students.insert("student_uuid".to_owned(), StudentInfo {
            uuid: "student_uuid".to_owned(),
            number: 3,
            fname: String::from(""),
            lname: String::from("")
        });

        let mut encrypted = db.encrypted.lock().unwrap();

        encrypted.insert("student_uuid".to_owned(), StudentEncrypted {
            uuid: "student_uuid".to_owned(),
            data: "first_name||last_name||pronouns".to_owned()
        });

        let mut residence = db.residences.lock().unwrap();

        residence.insert("student_uuid".to_owned(), StudentResidence {
            uuid: "student_uuid".to_owned(),
            hall: "Shecon".to_owned(),
            room: 212,
            wing: "Mario".to_owned()
        });

        let mut binding = db.activity_binds.lock().unwrap();

        binding.push(ActivityBind{
            student: "student_uuid".to_owned(),
            activity: "activity_uuid".to_owned()
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    let (activity, students) =match service.get_activity("activity_uuid", true, true).await {
        Ok(t) => t,
        Err(t) => {panic!("No activity found: {}", t.to_deobfuscated())},
    };

    assert!(activity.name == "Activity");
    assert!(students.len() == 1);
    assert!(students[0].number == 3);
    assert!(students[0].fname == "first_name");
}