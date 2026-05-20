use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuditLogResponse {
    pub id: i32,
    pub user_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub action: String,
    pub entity: String,
    pub entity_id: Option<i32>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: String,
}
