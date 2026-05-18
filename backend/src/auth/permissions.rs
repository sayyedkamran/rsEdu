use axum::http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    auth::utils::Claims,
    entities::{permissions, role_permissions, user_roles},
};

// Check action-based permission (what can this user do?)
async fn check_action_permission(
    db: &DatabaseConnection,
    user_id: i32,
    permission_name: &str,
) -> Result<(), (StatusCode, String)> {
    let user_role_records = user_roles::Entity::find()
        .filter(user_roles::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if user_role_records.is_empty() {
        return Err((StatusCode::FORBIDDEN, "No roles assigned to user".to_string()));
    }

    let role_ids: Vec<i32> = user_role_records
        .iter()
        .map(|ur| ur.role_id)
        .collect();

    let role_permission_records = role_permissions::Entity::find()
        .filter(role_permissions::Column::RoleId.is_in(role_ids))
        .all(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if role_permission_records.is_empty() {
        return Err((StatusCode::FORBIDDEN, "No permissions assigned to user roles".to_string()));
    }

    let permission_ids: Vec<i32> = role_permission_records
        .iter()
        .map(|rp| rp.permission_id)
        .collect();

    let permission = permissions::Entity::find()
        .filter(permissions::Column::Id.is_in(permission_ids))
        .filter(
            sea_orm::Condition::any()
                .add(permissions::Column::Name.eq(permission_name))
                .add(permissions::Column::Name.eq("*")),
        )
        .filter(permissions::Column::IsActive.eq(true))
        .one(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match permission {
        Some(_) => Ok(()),
        None => Err((
            StatusCode::FORBIDDEN,
            format!("Permission '{}' denied", permission_name),
        )),
    }
}

// Check resource scope (which data can this user access?)
fn check_resource_scope(
    claims: &Claims,
    resource_org_id: Option<i32>,
    resource_branch_id: Option<i32>,
) -> Result<(), (StatusCode, String)> {
    // Super admin — no org_id and no branch_id means access everything
    if claims.organization_id.is_none() && claims.branch_id.is_none() {
        return Ok(());
    }

    // Global resource — no org_id means shared data (provinces, cities etc.)
    if resource_org_id.is_none() && resource_branch_id.is_none() {
        return Ok(());
    }

    // Org admin — has org_id but no branch_id
    if claims.branch_id.is_none() {
        if claims.organization_id != resource_org_id {
            return Err((
                StatusCode::FORBIDDEN,
                "Access denied to this organization's data".to_string(),
            ));
        }
        return Ok(());
    }

    // Branch user — has both org_id and branch_id
    if claims.organization_id != resource_org_id {
        return Err((
            StatusCode::FORBIDDEN,
            "Access denied to this organization's data".to_string(),
        ));
    }

    if claims.branch_id != resource_branch_id {
        return Err((
            StatusCode::FORBIDDEN,
            "Access denied to this branch's data".to_string(),
        ));
    }

    Ok(())
}

// authorize — combines action permission + resource scope check
pub async fn authorize(
    db: &DatabaseConnection,
    claims: &Claims,
    permission_name: &str,
    resource_org_id: Option<i32>,
    resource_branch_id: Option<i32>,
) -> Result<(), (StatusCode, String)> {
    check_action_permission(
        db,
        claims.sub.parse::<i32>().unwrap_or(0),
        permission_name,
    )
    .await?;

    check_resource_scope(claims, resource_org_id, resource_branch_id)?;

    Ok(())
}

// Data scope — used for filtering list endpoints
pub struct DataScope {
    pub organization_id: Option<i32>,
    pub branch_id: Option<i32>,
}

pub fn get_data_scope(claims: &Claims) -> DataScope {
    DataScope {
        organization_id: claims.organization_id,
        branch_id: claims.branch_id,
    }
}