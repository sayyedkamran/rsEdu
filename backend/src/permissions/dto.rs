use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePermissionRequest {
    pub organization_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub module: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePermissionRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub module: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PermissionResponse {
    pub id: i32,
    pub organization_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub module: String,
    pub is_active: bool,
}
