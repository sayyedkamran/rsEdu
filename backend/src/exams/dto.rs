use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateExamRequest {
    pub branch_id: i32,
    pub class_id: i32,
    pub academic_year_id: i32,
    pub exam_type_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateExamRequest {
    pub class_id: Option<i32>,
    pub academic_year_id: Option<i32>,
    pub exam_type_id: Option<i32>,
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ExamResponse {
    pub id: i32,
    pub branch_id: i32,
    pub class_id: i32,
    pub academic_year_id: i32,
    pub exam_type_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub is_active: bool,
}
