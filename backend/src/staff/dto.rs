use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStaffRequest {
    pub user_id: i32,
    pub organization_id: i32,
    pub branch_id: Option<i32>,
    pub staff_type_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub father_name: Option<String>,
    pub date_of_birth: String,
    pub gender: String,
    pub cnic: Option<String>,
    pub joining_date: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStaffRequest {
    pub branch_id: Option<i32>,
    pub staff_type_id: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub father_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: Option<String>,
    pub cnic: Option<String>,
    pub joining_date: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StaffResponse {
    pub id: i32,
    pub user_id: i32,
    pub organization_id: i32,
    pub branch_id: Option<i32>,
    pub staff_type_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub father_name: Option<String>,
    pub date_of_birth: String,
    pub gender: String,
    pub cnic: Option<String>,
    pub joining_date: String,
    pub is_active: bool,
}
