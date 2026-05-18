use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateProvinceRequest {
    pub name: String,
    pub name_urdu: Option<String>,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProvinceRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub code: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ProvinceResponse {
    pub id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub code: String,
    pub is_active: bool,
}