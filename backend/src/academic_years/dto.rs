use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateAcademicYearRequest {
    pub organization_id: i32,
    pub stream_id: i32,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAcademicYearRequest {
    pub title: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AcademicYearResponse {
    pub id: i32,
    pub organization_id: i32,
    pub stream_id: i32,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub description: Option<String>,
    pub is_active: bool,
}