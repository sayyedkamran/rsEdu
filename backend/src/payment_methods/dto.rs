use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePaymentMethodRequest {
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub requires_reference: bool,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePaymentMethodRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub requires_reference: Option<bool>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PaymentMethodResponse {
    pub id: i32,
    pub organization_id: i32,
    pub name: String,
    pub name_urdu: Option<String>,
    pub requires_reference: bool,
    pub description: Option<String>,
    pub is_active: bool,
}
