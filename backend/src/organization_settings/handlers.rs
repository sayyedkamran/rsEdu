use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use chrono::Utc;

use crate::{
    AppState,
    auth::{utils::Claims, permissions::authorize, permissions::get_data_scope},
    entities::organization_settings::{self, ActiveModel},
    organization_settings::dto::{CreateOrganizationSettingRequest, OrganizationSettingResponse, UpdateOrganizationSettingRequest},
};

fn to_response(os: organization_settings::Model) -> OrganizationSettingResponse {
    OrganizationSettingResponse {
        id: os.id,
        organization_id: os.organization_id,
        key: os.key,
        value: os.value,
        description: os.description,
    }
}

// POST /api/v1/organization-settings
pub async fn create_organization_setting(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateOrganizationSettingRequest>,
) -> Result<Json<OrganizationSettingResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "organization_setting:create", Some(payload.organization_id), None).await?;

    let new_os = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        key: ActiveValue::Set(payload.key),
        value: ActiveValue::Set(payload.value),
        description: ActiveValue::Set(payload.description),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let os = new_os
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(os)))
}

// GET /api/v1/organization-settings
pub async fn get_organization_settings(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<OrganizationSettingResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "organization_setting:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = organization_settings::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(organization_settings::Column::OrganizationId.eq(org_id));
    }

    let records = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/organization-settings/{id}
pub async fn get_organization_setting(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<OrganizationSettingResponse>, (StatusCode, String)> {
    let os = organization_settings::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Organization setting not found".to_string()))?;

    authorize(&*state.db, &claims, "organization_setting:read", Some(os.organization_id), None).await?;

    Ok(Json(to_response(os)))
}

// PUT /api/v1/organization-settings/{id}
pub async fn update_organization_setting(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateOrganizationSettingRequest>,
) -> Result<Json<OrganizationSettingResponse>, (StatusCode, String)> {
    let os = organization_settings::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Organization setting not found".to_string()))?;

    authorize(&*state.db, &claims, "organization_setting:update", Some(os.organization_id), None).await?;

    let mut active_model: ActiveModel = os.into();

    if let Some(value) = payload.value {
        active_model.value = ActiveValue::Set(value);
    }
    if let Some(description) = payload.description {
        active_model.description = ActiveValue::Set(Some(description));
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/organization-settings/{id}
pub async fn delete_organization_setting(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let os = organization_settings::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Organization setting not found".to_string()))?;

    authorize(&*state.db, &claims, "organization_setting:delete", Some(os.organization_id), None).await?;

    let active_model: ActiveModel = os.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
