use std::sync::Arc;

use hsp_backend::{encryption::PlaintextEncryption, repository::{Repository, activities_repository::{Activity, ActivityBind}, student_repository::StudentInfo}, service::ActivitiesService};
use hsp_backend::database::MockDB;


#[tokio::test]
async fn bind_activity_success() {
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
            student: "Anissa".to_owned(),
            activity: "activity_uuid".to_owned()
        });

        let mut studentinfo = db.students.lock().unwrap();

        studentinfo.insert("Anissa".to_owned(), StudentInfo{
            uuid: "Anissa".to_owned(),
            number: 12,
            fname: "".to_owned(),
            lname: "".to_owned(),
        });

        studentinfo.insert("Oliver".to_owned(), StudentInfo{
            uuid: "Oliver".to_owned(),
            number: 13,
            fname: "".to_owned(),
            lname: "".to_owned(),
        });
    }

    let arc_db: Arc<dyn Repository> = Arc::new(db);
    let service = ActivitiesService::new(Arc::clone(&arc_db), Arc::new(enc));
    
    match service.bind_students("activity_uuid", vec![12]).await {
        Ok(()) => {},
        Err(t) => panic!("{}", t.to_deobfuscated())
    };

    {
        let downcast = (*arc_db).downcast_ref::<MockDB>().unwrap();
        let activity_binds = downcast.activity_binds.lock().unwrap();

        assert_eq!(activity_binds.len(), 1);
        assert_eq!(activity_binds[0].student, "Anissa");
        
    }
}