use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub organization_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub profile_picture_path: Option<String>,
    pub profile_picture_url: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfilePictureRequest {
    pub profile_picture_path: String,
}