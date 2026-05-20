use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateBranchContactRequest {
    pub branch_id: i32,
    pub contact_type: String,
    pub value: String,
    pub has_whatsapp: bool,
    pub is_primary: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBranchContactRequest {
    pub contact_type: Option<String>,
    pub value: Option<String>,
    pub has_whatsapp: Option<bool>,
    pub is_primary: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BranchContactResponse {
    pub id: i32,
    pub branch_id: i32,
    pub contact_type: String,
    pub value: String,
    pub has_whatsapp: bool,
    pub is_primary: bool,
    pub is_active: bool,
}
