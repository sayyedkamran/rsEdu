use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateFeeBillRequest {
    pub student_id: i32,
    pub branch_id: i32,
    pub academic_year_id: i32,
    pub fee_type_id: i32,
    pub bill_month: i32,
    pub bill_year: i32,
    pub amount: i32,
    pub discount_amount: i32,
    pub late_fee: i32,
    pub carry_forward: i32,
    pub net_amount: i32,
    pub amount_paid: i32,
    pub balance: i32,
    pub status: String,
    pub due_date: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFeeBillRequest {
    pub discount_amount: Option<i32>,
    pub late_fee: Option<i32>,
    pub carry_forward: Option<i32>,
    pub net_amount: Option<i32>,
    pub amount_paid: Option<i32>,
    pub balance: Option<i32>,
    pub status: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FeeBillResponse {
    pub id: i32,
    pub student_id: i32,
    pub branch_id: i32,
    pub academic_year_id: i32,
    pub fee_type_id: i32,
    pub bill_month: i32,
    pub bill_year: i32,
    pub amount: i32,
    pub discount_amount: i32,
    pub late_fee: i32,
    pub carry_forward: i32,
    pub net_amount: i32,
    pub amount_paid: i32,
    pub balance: i32,
    pub status: String,
    pub due_date: String,
    pub generated_at: String,
}
