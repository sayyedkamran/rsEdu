use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateSalaryPaymentRequest {
    pub staff_monthly_salary_id: i32,
    pub staff_id: i32,
    pub amount_paid: i32,
    pub payment_method_id: i32,
    pub reference_number: Option<String>,
    pub payment_date: String,
    pub receipt_number: String,
    pub remarks: Option<String>,
    pub paid_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSalaryPaymentRequest {
    pub reference_number: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SalaryPaymentResponse {
    pub id: i32,
    pub staff_monthly_salary_id: i32,
    pub staff_id: i32,
    pub amount_paid: i32,
    pub payment_method_id: i32,
    pub reference_number: Option<String>,
    pub payment_date: String,
    pub receipt_number: String,
    pub remarks: Option<String>,
    pub paid_by: i32,
}
