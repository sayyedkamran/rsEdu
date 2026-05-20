use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStudentEnrollmentRequest {
    pub student_id: i32,
    pub class_id: i32,
    pub academic_year_id: i32,
    pub roll_number: Option<String>,
    pub enrollment_date: String,
    pub status: String,
    pub remarks: Option<String>,
    pub enrolled_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudentEnrollmentRequest {
    pub roll_number: Option<String>,
    pub status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StudentEnrollmentResponse {
    pub id: i32,
    pub student_id: i32,
    pub class_id: i32,
    pub academic_year_id: i32,
    pub roll_number: Option<String>,
    pub enrollment_date: String,
    pub status: String,
    pub remarks: Option<String>,
    pub enrolled_by: i32,
}
