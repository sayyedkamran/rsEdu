use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStaffMonthlySalaryDetailRequest {
    pub staff_monthly_salary_id: i32,
    pub allowance_deduction_type_id: Option<i32>,
    pub description: String,
    pub r#type: String,
    pub amount: i32,
    pub is_adhoc: bool,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStaffMonthlySalaryDetailRequest {
    pub description: Option<String>,
    pub amount: Option<i32>,
    pub is_adhoc: Option<bool>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StaffMonthlySalaryDetailResponse {
    pub id: i32,
    pub staff_monthly_salary_id: i32,
    pub allowance_deduction_type_id: Option<i32>,
    pub description: String,
    pub r#type: String,
    pub amount: i32,
    pub is_adhoc: bool,
    pub remarks: Option<String>,
}
