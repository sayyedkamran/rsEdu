use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStudentRequest {
    pub user_id: i32,
    pub organization_id: i32,
    pub branch_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub roll_number: String,
    pub admission_date: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudentRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub roll_number: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StudentResponse {
    pub id: i32,
    pub user_id: i32,
    pub organization_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub roll_number: String,
    pub admission_date: String,
    pub is_active: bool,
}