use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStudentScholarshipRequest {
    pub student_id: i32,
    pub scholarship_id: i32,
    pub academic_year_id: i32,
    pub start_date: String,
    pub end_date: Option<String>,
    pub status: String,
    pub requested_by: i32,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudentScholarshipRequest {
    pub end_date: Option<String>,
    pub status: Option<String>,
    pub approved_by: Option<i32>,
    pub remarks: Option<String>,
    pub rejection_reason: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StudentScholarshipResponse {
    pub id: i32,
    pub student_id: i32,
    pub scholarship_id: i32,
    pub academic_year_id: i32,
    pub start_date: String,
    pub end_date: Option<String>,
    pub status: String,
    pub requested_by: i32,
    pub approved_by: Option<i32>,
    pub remarks: Option<String>,
    pub rejection_reason: Option<String>,
    pub is_active: bool,
}
