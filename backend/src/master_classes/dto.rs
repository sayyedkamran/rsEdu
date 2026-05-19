use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMasterClassRequest {
    pub organization_id: i32,
    pub stream_id: i32,
    pub class_level_id: Option<i32>,
    pub name: String,
    pub name_urdu: Option<String>,
    pub order: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMasterClassRequest {
    pub stream_id: Option<i32>,
    pub class_level_id: Option<i32>,
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct MasterClassResponse {
    pub id: i32,
    pub organization_id: i32,
    pub stream_id: i32,
    pub class_level_id: Option<i32>,
    pub name: String,
    pub name_urdu: Option<String>,
    pub order: i32,
    pub is_active: bool,
}