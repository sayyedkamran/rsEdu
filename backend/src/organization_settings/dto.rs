use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateOrganizationSettingRequest {
    pub organization_id: i32,
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrganizationSettingRequest {
    pub value: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationSettingResponse {
    pub id: i32,
    pub organization_id: i32,
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}
