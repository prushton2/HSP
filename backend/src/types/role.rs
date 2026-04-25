#[derive(Debug, PartialEq, Clone)]
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

impl From<&Role> for String {
    fn from(value: &Role) -> Self {
        match value {
            Role::Admin => "Admin".to_owned(),
            Role::Owner => "Owner".to_owned(),
            Role::Staff => "Staff".to_owned()
        }
    }
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "Admin" => Role::Admin,
            "Owner" => Role::Owner,
            _       => Role::Staff,
        }   
    }
}

impl From<&i32> for Role {
    fn from(value: &i32) -> Self {
        match value {
            1 => Role::Admin,
            2 => Role::Owner,
            _ => Role::Staff,
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