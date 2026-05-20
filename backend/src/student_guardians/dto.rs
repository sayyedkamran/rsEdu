use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStudentGuardianRequest {
    pub student_id: i32,
    pub guardian_id: i32,
    pub is_primary_contact: bool,
    pub is_emergency_contact: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudentGuardianRequest {
    pub is_primary_contact: Option<bool>,
    pub is_emergency_contact: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StudentGuardianResponse {
    pub id: i32,
    pub student_id: i32,
    pub guardian_id: i32,
    pub is_primary_contact: bool,
    pub is_emergency_contact: bool,
}
