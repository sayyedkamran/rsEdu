use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateBranchRequest {
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub code: String,
    pub city_id: Option<i32>,
    pub area: Option<String>,
    pub address_line: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBranchRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub code: Option<String>,
    pub city_id: Option<i32>,
    pub area: Option<String>,
    pub address_line: Option<String>,
    pub postal_code: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BranchResponse {
    pub id: i32,
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub code: String,
    pub city_id: Option<i32>,
    pub area: Option<String>,
    pub address_line: Option<String>,
    pub postal_code: Option<String>,
    pub is_active: bool,
}