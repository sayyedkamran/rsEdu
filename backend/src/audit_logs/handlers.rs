use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    AppState,
    auth::{utils::Claims, permissions::authorize, permissions::get_data_scope},
    entities::audit_logs,
    audit_logs::dto::AuditLogResponse,
};

fn to_response(al: audit_logs::Model) -> AuditLogResponse {
    AuditLogResponse {
        id: al.id,
        user_id: al.user_id,
        organization_id: al.organization_id,
        branch_id: al.branch_id,
        action: al.action,
        entity: al.entity,
        entity_id: al.entity_id,
        old_value: al.old_value,
        new_value: al.new_value,
        ip_address: al.ip_address,
        created_at: al.created_at.to_string(),
    }
}

// GET /api/v1/audit-logs
pub async fn get_audit_logs(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<AuditLogResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "audit_log:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = audit_logs::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(audit_logs::Column::OrganizationId.eq(org_id));
    }
    if let Some(branch_id) = scope.branch_id {
        query = query.filter(audit_logs::Column::BranchId.eq(branch_id));
    }

    let records = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/audit-logs/{id}
pub async fn get_audit_log(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<AuditLogResponse>, (StatusCode, String)> {
    let al = audit_logs::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Audit log not found".to_string()))?;

    authorize(&*state.db, &claims, "audit_log:read", al.organization_id, al.branch_id).await?;

    Ok(Json(to_response(al)))
}
