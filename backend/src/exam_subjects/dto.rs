use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateExamSubjectRequest {
    pub exam_id: i32,
    pub subject_id: i32,
    pub total_marks: i32,
    pub passing_marks: i32,
    pub exam_date: String,
    pub exam_time: Option<String>,
    pub venue: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateExamSubjectRequest {
    pub total_marks: Option<i32>,
    pub passing_marks: Option<i32>,
    pub exam_date: Option<String>,
    pub exam_time: Option<String>,
    pub venue: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ExamSubjectResponse {
    pub id: i32,
    pub exam_id: i32,
    pub subject_id: i32,
    pub total_marks: i32,
    pub passing_marks: i32,
    pub exam_date: String,
    pub exam_time: Option<String>,
    pub venue: Option<String>,
    pub is_active: bool,
}
