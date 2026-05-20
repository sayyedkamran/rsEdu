use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateSalaryStructureRequest {
    pub organization_id: i32,
    pub branch_id: Option<i32>,
    pub name: String,
    pub name_urdu: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSalaryStructureRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SalaryStructureResponse {
    pub id: i32,
    pub organization_id: i32,
    pub branch_id: Option<i32>,
    pub name: String,
    pub name_urdu: Option<String>,
    pub description: Option<String>,
    pub is_active: bool,
}
