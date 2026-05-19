use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateTransferRequest {
    pub entity_type: String,
    pub entity_id: i32,
    pub from_branch_id: i32,
    pub to_branch_id: i32,
    pub transfer_date: String,
    pub reason: Option<String>,
    pub requested_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTransferRequest {
    pub transfer_date: Option<String>,
    pub reason: Option<String>,
    pub approved_by: Option<i32>,
    pub status: Option<String>,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransferResponse {
    pub id: i32,
    pub entity_type: String,
    pub entity_id: i32,
    pub from_branch_id: i32,
    pub to_branch_id: i32,
    pub transfer_date: String,
    pub reason: Option<String>,
    pub requested_by: i32,
    pub approved_by: Option<i32>,
    pub status: String,
    pub rejection_reason: Option<String>,
}
