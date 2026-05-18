use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateCityRequest {
    pub name: String,
    pub name_urdu: Option<String>,
    pub province_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCityRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CityResponse {
    pub id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub province_id: i32,
    pub is_active: bool,
}