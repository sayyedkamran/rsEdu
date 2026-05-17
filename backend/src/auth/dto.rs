use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role_id: i32,
    pub organization_id: i32,
    pub branch_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub username: String,
    pub role: String,
    pub organization_id: Option<i32>,
    pub branch_id: Option<i32>,
}
