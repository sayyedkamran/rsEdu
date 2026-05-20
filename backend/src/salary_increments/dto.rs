use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateSalaryIncrementRequest {
    pub staff_id: i32,
    pub staff_salary_id: i32,
    pub increment_type: String,
    pub previous_basic_salary: i32,
    pub new_basic_salary: i32,
    pub increment_amount: i32,
    pub effective_from: String,
    pub reason: Option<String>,
    pub approved_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSalaryIncrementRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SalaryIncrementResponse {
    pub id: i32,
    pub staff_id: i32,
    pub staff_salary_id: i32,
    pub increment_type: String,
    pub previous_basic_salary: i32,
    pub new_basic_salary: i32,
    pub increment_amount: i32,
    pub effective_from: String,
    pub reason: Option<String>,
    pub approved_by: i32,
}
