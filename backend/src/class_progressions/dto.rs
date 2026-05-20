use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateClassProgressionRequest {
    pub from_class_id: i32,
    pub to_class_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClassProgressionRequest {
    pub from_class_id: Option<i32>,
    pub to_class_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ClassProgressionResponse {
    pub id: i32,
    pub from_class_id: i32,
    pub to_class_id: i32,
}
