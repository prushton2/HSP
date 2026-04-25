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

#[derive(Debug, PartialEq)]
pub enum Role {
    Staff,
    Admin,
    Owner
}

impl From<&Role> for i32 {
    fn from(value: &Role) -> i32 {
        match value {
            Role::Staff => 0,
            Role::Admin => 1,
            Role::Owner => 2,
        }
    }
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "Admin" => Role::Admin,
            "Owner" => Role::Owner,
            _ => Role::Staff,
        }   
    }
}

impl From<&Role> for String {
    fn from(value: &Role) -> Self {
        match value {
            Role::Admin => "Admin".to_owned(),
            Role::Owner => "Owner".to_owned(),
            Role::Staff => "Staff".to_owned()
        }
    }
}

impl PartialOrd for Role {
    fn ge(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Greater) || 
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)        
    }
    fn gt(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Greater)
    }
    fn le(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Less) || 
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
    fn lt(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Less)
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if i32::from(self) == i32::from(other) {
            return Some(std::cmp::Ordering::Equal);
        }

        if i32::from(self) > i32::from(other) {
            return Some(std::cmp::Ordering::Greater);
        }

        if i32::from(self) < i32::from(other) {
            return Some(std::cmp::Ordering::Less);
        }

        return None
    }
}

pub struct UserInfo {
    pub first_name: String,
    pub last_name: String,
    pub role: Role,
    pub accessed_from: String
}