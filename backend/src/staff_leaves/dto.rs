use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateStaffLeaveRequest {
    pub staff_id: i32,
    pub branch_id: i32,
    pub leave_type_id: i32,
    pub start_date: String,
    pub end_date: String,
    pub days: i32,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStaffLeaveRequest {
    pub status: Option<String>,
    pub approved_by: Option<i32>,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct StaffLeaveResponse {
    pub id: i32,
    pub staff_id: i32,
    pub branch_id: i32,
    pub leave_type_id: i32,
    pub start_date: String,
    pub end_date: String,
    pub days: i32,
    pub reason: Option<String>,
    pub status: String,
    pub approved_by: Option<i32>,
    pub rejection_reason: Option<String>,
}
