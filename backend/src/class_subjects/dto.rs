use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateClassSubjectRequest {
    pub class_id: i32,
    pub subject_id: i32,
    pub staff_id: Option<i32>,
    pub weekly_periods: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClassSubjectRequest {
    pub staff_id: Option<i32>,
    pub weekly_periods: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ClassSubjectResponse {
    pub id: i32,
    pub class_id: i32,
    pub subject_id: i32,
    pub staff_id: Option<i32>,
    pub weekly_periods: Option<i32>,
    pub is_active: bool,
}
