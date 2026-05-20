use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateLeaveTypeRequest {
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub max_days_per_year: i32,
    pub is_paid: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLeaveTypeRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub max_days_per_year: Option<i32>,
    pub is_paid: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct LeaveTypeResponse {
    pub id: i32,
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub max_days_per_year: i32,
    pub is_paid: bool,
    pub is_active: bool,
}
