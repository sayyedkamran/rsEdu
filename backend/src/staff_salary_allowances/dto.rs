use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStaffSalaryAllowanceRequest {
    pub staff_salary_id: i32,
    pub allowance_deduction_type_id: i32,
    pub amount: i32,
    pub calculation_type: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStaffSalaryAllowanceRequest {
    pub amount: Option<i32>,
    pub calculation_type: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StaffSalaryAllowanceResponse {
    pub id: i32,
    pub staff_salary_id: i32,
    pub allowance_deduction_type_id: i32,
    pub amount: i32,
    pub calculation_type: String,
    pub is_active: bool,
}
