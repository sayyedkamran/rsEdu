use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStudentRequest {
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub father_name: Option<String>,
    pub date_of_birth: String,
    pub gender: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub class: String,
    pub section: String,
    pub roll_number: String,
    pub admission_date: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudentRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub father_name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub class: Option<String>,
    pub section: Option<String>,
    pub roll_number: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StudentResponse {
    pub id: i32,
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub father_name: Option<String>,
    pub date_of_birth: String,
    pub gender: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub class: String,
    pub section: String,
    pub roll_number: String,
    pub admission_date: String,
    pub is_active: bool,
}