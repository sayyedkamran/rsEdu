use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateClassRequest {
    pub branch_id: i32,
    pub master_class_id: i32,
    pub master_section_id: i32,
    pub academic_year_id: i32,
    pub class_staff_id: Option<i32>,
    pub capacity: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClassRequest {
    pub master_class_id: Option<i32>,
    pub master_section_id: Option<i32>,
    pub academic_year_id: Option<i32>,
    pub class_staff_id: Option<i32>,
    pub capacity: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ClassResponse {
    pub id: i32,
    pub branch_id: i32,
    pub master_class_id: i32,
    pub master_section_id: i32,
    pub academic_year_id: i32,
    pub class_staff_id: Option<i32>,
    pub capacity: Option<i32>,
    pub is_active: bool,
}
