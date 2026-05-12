use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTeacherRequest {
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub father_name: Option<String>,
    pub date_of_birth: String,
    pub gender: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub qualification: String,
    pub specialization: String,
    pub joining_date: String,
    pub cnic: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTeacherRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub father_name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub qualification: Option<String>,
    pub specialization: Option<String>,
    pub cnic: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TeacherResponse {
    pub id: i32,
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub father_name: Option<String>,
    pub date_of_birth: String,
    pub gender: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub qualification: String,
    pub specialization: String,
    pub joining_date: String,
    pub cnic: Option<String>,
    pub is_active: bool,
}