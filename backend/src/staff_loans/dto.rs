use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStaffLoanRequest {
    pub staff_id: i32,
    pub branch_id: i32,
    pub amount: i32,
    pub purpose: Option<String>,
    pub installments: i32,
    pub installment_amount: i32,
    pub start_month: i32,
    pub start_year: i32,
    pub approved_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStaffLoanRequest {
    pub purpose: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StaffLoanResponse {
    pub id: i32,
    pub staff_id: i32,
    pub branch_id: i32,
    pub amount: i32,
    pub purpose: Option<String>,
    pub installments: i32,
    pub installment_amount: i32,
    pub start_month: i32,
    pub start_year: i32,
    pub status: String,
    pub approved_by: i32,
}
