use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStaffLoanDeductionRequest {
    pub staff_loan_id: i32,
    pub staff_monthly_salary_id: i32,
    pub amount_deducted: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStaffLoanDeductionRequest {
    pub amount_deducted: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct StaffLoanDeductionResponse {
    pub id: i32,
    pub staff_loan_id: i32,
    pub staff_monthly_salary_id: i32,
    pub amount_deducted: i32,
}
