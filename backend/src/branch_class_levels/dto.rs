use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateBranchClassLevelRequest {
    pub branch_id: i32,
    pub class_level_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBranchClassLevelRequest {
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct BranchClassLevelResponse {
    pub id: i32,
    pub branch_id: i32,
    pub class_level_id: i32,
    pub is_active: bool,
}
