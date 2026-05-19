use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateFeePaymentRequest {
    pub fee_bill_id: i32,
    pub student_id: i32,
    pub payment_method_id: i32,
    pub amount_paid: i32,
    pub reference_number: Option<String>,
    pub payment_date: String,
    pub receipt_number: String,
    pub remarks: Option<String>,
    pub received_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFeePaymentRequest {
    pub reference_number: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FeePaymentResponse {
    pub id: i32,
    pub fee_bill_id: i32,
    pub student_id: i32,
    pub payment_method_id: i32,
    pub amount_paid: i32,
    pub reference_number: Option<String>,
    pub payment_date: String,
    pub receipt_number: String,
    pub remarks: Option<String>,
    pub received_by: i32,
}
