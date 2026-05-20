use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateAddressRequest {
    pub entity_type: String,
    pub entity_id: i32,
    pub address_type: String,
    pub address_line: String,
    pub area: Option<String>,
    pub city_id: Option<i32>,
    pub postal_code: Option<String>,
    pub is_primary: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAddressRequest {
    pub address_type: Option<String>,
    pub address_line: Option<String>,
    pub area: Option<String>,
    pub city_id: Option<i32>,
    pub postal_code: Option<String>,
    pub is_primary: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AddressResponse {
    pub id: i32,
    pub entity_type: String,
    pub entity_id: i32,
    pub address_type: String,
    pub address_line: String,
    pub area: Option<String>,
    pub city_id: Option<i32>,
    pub postal_code: Option<String>,
    pub is_primary: bool,
    pub is_active: bool,
}
