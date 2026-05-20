use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStudentDiscountRequest {
    pub student_id: i32,
    pub discount_id: Option<i32>,
    pub academic_year_id: i32,
    pub fee_type_id: Option<i32>,
    pub discount_type: String,
    pub value: i32,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub reason: Option<String>,
    pub status: String,
    pub requested_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudentDiscountRequest {
    pub discount_type: Option<String>,
    pub value: Option<i32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub reason: Option<String>,
    pub status: Option<String>,
    pub approved_by: Option<i32>,
    pub rejection_reason: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StudentDiscountResponse {
    pub id: i32,
    pub student_id: i32,
    pub discount_id: Option<i32>,
    pub academic_year_id: i32,
    pub fee_type_id: Option<i32>,
    pub discount_type: String,
    pub value: i32,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub reason: Option<String>,
    pub status: String,
    pub requested_by: i32,
    pub approved_by: Option<i32>,
    pub rejection_reason: Option<String>,
    pub is_active: bool,
}
