use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStaffTypeRequest {
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub is_teaching: bool,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStaffTypeRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub is_teaching: Option<bool>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StaffTypeResponse {
    pub id: i32,
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub is_teaching: bool,
    pub description: Option<String>,
    pub is_active: bool,
}
