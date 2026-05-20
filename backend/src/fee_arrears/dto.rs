use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateFeeArrearRequest {
    pub student_id: i32,
    pub branch_id: i32,
    pub academic_year_id: i32,
    pub amount: i32,
    pub reason: String,
    pub status: String,
    pub entered_by: i32,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFeeArrearRequest {
    pub amount: Option<i32>,
    pub reason: Option<String>,
    pub status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FeeArrearResponse {
    pub id: i32,
    pub student_id: i32,
    pub branch_id: i32,
    pub academic_year_id: i32,
    pub amount: i32,
    pub reason: String,
    pub status: String,
    pub entered_by: i32,
    pub remarks: Option<String>,
}
