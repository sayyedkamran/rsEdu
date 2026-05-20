use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateAllowanceDeductionTypeRequest {
    pub organization_id: i32,
    pub branch_id: Option<i32>,
    pub name: String,
    pub name_urdu: Option<String>,
    pub r#type: String,
    pub occurrence: String,
    pub frequency: Option<String>,
    pub is_taxable: bool,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAllowanceDeductionTypeRequest {
    pub name: Option<String>,
    pub name_urdu: Option<String>,
    pub r#type: Option<String>,
    pub occurrence: Option<String>,
    pub frequency: Option<String>,
    pub is_taxable: Option<bool>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AllowanceDeductionTypeResponse {
    pub id: i32,
    pub organization_id: i32,
    pub branch_id: Option<i32>,
    pub name: String,
    pub name_urdu: Option<String>,
    pub r#type: String,
    pub occurrence: String,
    pub frequency: Option<String>,
    pub is_taxable: bool,
    pub description: Option<String>,
    pub is_active: bool,
}
