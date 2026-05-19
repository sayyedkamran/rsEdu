use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateAttendanceRequest {
    pub branch_id: i32,
    pub class_id: i32,
    pub student_id: i32,
    pub marked_by: i32,
    pub date: String,
    pub status: String,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAttendanceRequest {
    pub status: Option<String>,
    pub remarks: Option<String>,
    pub marked_by: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct AttendanceResponse {
    pub id: i32,
    pub branch_id: i32,
    pub class_id: i32,
    pub student_id: i32,
    pub marked_by: i32,
    pub date: String,
    pub status: String,
    pub remarks: Option<String>,
}
