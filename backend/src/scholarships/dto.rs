use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateScholarshipRequest {
    pub organization_id: i32,
    pub fee_type_id: Option<i32>,
    pub name: String,
    pub name_urdu: Option<String>,
    pub coverage_type: String,
    pub value: i32,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateScholarshipRequest {
    pub fee_type_id: Option<i32>,
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub coverage_type: Option<String>,
    pub value: Option<i32>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ScholarshipResponse {
    pub id: i32,
    pub organization_id: i32,
    pub fee_type_id: Option<i32>,
    pub name: String,
    pub name_urdu: Option<String>,
    pub coverage_type: String,
    pub value: i32,
    pub description: Option<String>,
    pub is_active: bool,
}
