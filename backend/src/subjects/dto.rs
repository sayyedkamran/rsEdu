use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateSubjectRequest {
    pub organization_id: i32,
    pub stream_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub code: String,
    pub medium: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubjectRequest {
    pub stream_id: Option<i32>,
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub code: Option<String>,
    pub medium: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SubjectResponse {
    pub id: i32,
    pub organization_id: i32,
    pub stream_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub code: String,
    pub medium: String,
    pub description: Option<String>,
    pub is_active: bool,
}
