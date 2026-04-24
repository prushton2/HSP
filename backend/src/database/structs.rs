use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct AllStudentInfo {
    pub first_name: String,
    pub last_name: String,
    pub pronouns: String,
    pub info: TableStudentInfo,
    pub residence: TableResidencies
}



impl Default for AllStudentInfo {
    fn default() -> Self {
        Self {
            first_name: "".to_string(),
            last_name: "".to_string(),
            pronouns: "".to_string(),
            info: TableStudentInfo::default(),
            residence: TableResidencies::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableStudentInfo {
    pub uuid: String,
    pub number: i32,
}

impl Default for TableStudentInfo {
    fn default() -> Self {
        Self {
            uuid: "".to_string(),
            number: -1
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableResidencies {
    pub uuid: String,
    pub hall: String,
    pub room: i32,
    pub wing: String,
    pub role: String,
}

impl Default for TableResidencies {
    fn default() -> Self {
        Self {
            uuid: "".to_string(),
            hall: "".to_string(),
            room: -1,
            wing: "".to_string(),
            role: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableStudentActivities {
    pub uuid: String,
    pub date: chrono::NaiveDate,
    pub activity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableActivities {
    pub activity: String,
    pub date: chrono::NaiveDate,
    pub staff: Vec<String>
}