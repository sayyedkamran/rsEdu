use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateExamTypeRequest {
    pub organization_id: i32,
    pub stream_id: Option<i32>,
    pub master_class_id: Option<i32>,
    pub name: String,
    pub name_urdu: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateExamTypeRequest {
    pub stream_id: Option<i32>,
    pub master_class_id: Option<i32>,
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ExamTypeResponse {
    pub id: i32,
    pub organization_id: i32,
    pub stream_id: Option<i32>,
    pub master_class_id: Option<i32>,
    pub name: String,
    pub name_urdu: Option<String>,
    pub description: Option<String>,
    pub is_active: bool,
}
