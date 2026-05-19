use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateResultRequest {
    pub exam_subject_id: i32,
    pub student_id: i32,
    pub marks_obtained: Option<i32>,
    pub is_absent: bool,
    pub is_exempt: bool,
    pub grade: Option<String>,
    pub remarks: Option<String>,
    pub entered_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateResultRequest {
    pub marks_obtained: Option<i32>,
    pub is_absent: Option<bool>,
    pub is_exempt: Option<bool>,
    pub grade: Option<String>,
    pub remarks: Option<String>,
    pub entered_by: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ResultResponse {
    pub id: i32,
    pub exam_subject_id: i32,
    pub student_id: i32,
    pub marks_obtained: Option<i32>,
    pub is_absent: bool,
    pub is_exempt: bool,
    pub grade: Option<String>,
    pub remarks: Option<String>,
    pub entered_by: i32,
}
