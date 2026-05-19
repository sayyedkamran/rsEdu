use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateDiscountRequest {
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub discount_type: String,
    pub value: i32,
    pub description: Option<String>,
    pub requires_approval: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDiscountRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub discount_type: Option<String>,
    pub value: Option<i32>,
    pub description: Option<String>,
    pub requires_approval: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct DiscountResponse {
    pub id: i32,
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub discount_type: String,
    pub value: i32,
    pub description: Option<String>,
    pub requires_approval: bool,
    pub is_active: bool,
}
