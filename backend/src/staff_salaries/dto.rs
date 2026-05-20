use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStaffSalaryRequest {
    pub staff_id: i32,
    pub salary_structure_id: i32,
    pub basic_salary: i32,
    pub effective_from: String,
    pub effective_to: Option<String>,
    pub remarks: Option<String>,
    pub assigned_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStaffSalaryRequest {
    pub basic_salary: Option<i32>,
    pub effective_to: Option<String>,
    pub remarks: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StaffSalaryResponse {
    pub id: i32,
    pub staff_id: i32,
    pub salary_structure_id: i32,
    pub basic_salary: i32,
    pub effective_from: String,
    pub effective_to: Option<String>,
    pub remarks: Option<String>,
    pub assigned_by: i32,
    pub is_active: bool,
}
