use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateGuardianRequest {
    pub user_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub relationship: String,
    pub cnic: Option<String>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGuardianRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub relationship: Option<String>,
    pub cnic: Option<String>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GuardianResponse {
    pub id: i32,
    pub user_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub relationship: String,
    pub cnic: Option<String>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
    pub is_active: bool,
}
