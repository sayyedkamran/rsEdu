use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateOrganizationRequest {
    pub name: String,
    pub name_urdu: Option<String>,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub city_id: Option<i32>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrganizationRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub city_id: Option<i32>,
    pub address: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationResponse {
    pub id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub city_id: Option<i32>,
    pub address: Option<String>,
    pub is_active: bool,
}