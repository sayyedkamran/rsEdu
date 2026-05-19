use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateFeeStructureRequest {
    pub organization_id: i32,
    pub branch_id: Option<i32>,
    pub class_level_id: Option<i32>,
    pub master_class_id: Option<i32>,
    pub student_id: Option<i32>,
    pub fee_type_id: i32,
    pub academic_year_id: i32,
    pub amount: i32,
    pub due_day: i32,
    pub bill_generation_frequency: String,
    pub late_fee_type: Option<String>,
    pub late_fee_value: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFeeStructureRequest {
    pub branch_id: Option<i32>,
    pub class_level_id: Option<i32>,
    pub master_class_id: Option<i32>,
    pub student_id: Option<i32>,
    pub fee_type_id: Option<i32>,
    pub academic_year_id: Option<i32>,
    pub amount: Option<i32>,
    pub due_day: Option<i32>,
    pub bill_generation_frequency: Option<String>,
    pub late_fee_type: Option<String>,
    pub late_fee_value: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct FeeStructureResponse {
    pub id: i32,
    pub organization_id: i32,
    pub branch_id: Option<i32>,
    pub class_level_id: Option<i32>,
    pub master_class_id: Option<i32>,
    pub student_id: Option<i32>,
    pub fee_type_id: i32,
    pub academic_year_id: i32,
    pub amount: i32,
    pub due_day: i32,
    pub bill_generation_frequency: String,
    pub late_fee_type: Option<String>,
    pub late_fee_value: Option<i32>,
    pub is_active: bool,
}
