use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStreamRequest {
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStreamRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct StreamResponse {
    pub id: i32,
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub description: Option<String>,
    pub is_active: bool,
}