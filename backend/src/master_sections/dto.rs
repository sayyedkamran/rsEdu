use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMasterSectionRequest {
    pub organization_id: i32,
    pub name: String,
    pub letter: String,
    pub name_urdu: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMasterSectionRequest {
    pub name: Option<String>,
    pub letter: Option<String>,
    pub name_urdu: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct MasterSectionResponse {
    pub id: i32,
    pub organization_id: i32,
    pub name: String,
    pub letter: String,
    pub name_urdu: Option<String>,
    pub is_active: bool,
}