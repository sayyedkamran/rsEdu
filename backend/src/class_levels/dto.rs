use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateClassLevelRequest {
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub order: i32,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClassLevelRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub order: Option<i32>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ClassLevelResponse {
    pub id: i32,
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub order: i32,
    pub description: Option<String>,
    pub is_active: bool,
}