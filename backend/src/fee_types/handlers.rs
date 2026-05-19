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
    entities::fee_types::{self, ActiveModel},
    fee_types::dto::{CreateFeeTypeRequest, FeeTypeResponse, UpdateFeeTypeRequest},
};

fn to_response(ft: fee_types::Model) -> FeeTypeResponse {
    FeeTypeResponse {
        id: ft.id,
        organization_id: ft.organization_id,
        name: ft.name,
        name_urdu: ft.name_urdu,
        recurrence: ft.recurrence,
        description: ft.description,
        is_active: ft.is_active,
    }
}

// POST /api/v1/fee-types
pub async fn create_fee_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateFeeTypeRequest>,
) -> Result<Json<FeeTypeResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "fee_type:create", Some(payload.organization_id), None).await?;

    let new_ft = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        recurrence: ActiveValue::Set(payload.recurrence),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let ft = new_ft
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(ft)))
}

// GET /api/v1/fee-types
pub async fn get_fee_types(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<FeeTypeResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "fee_type:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = fee_types::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(fee_types::Column::OrganizationId.eq(org_id));
    }

    let fee_types = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(fee_types.into_iter().map(to_response).collect()))
}

// GET /api/v1/fee-types/{id}
pub async fn get_fee_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<FeeTypeResponse>, (StatusCode, String)> {
    let ft = fee_types::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee type not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_type:read", Some(ft.organization_id), None).await?;

    Ok(Json(to_response(ft)))
}

// PUT /api/v1/fee-types/{id}
pub async fn update_fee_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateFeeTypeRequest>,
) -> Result<Json<FeeTypeResponse>, (StatusCode, String)> {
    let ft = fee_types::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee type not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_type:update", Some(ft.organization_id), None).await?;

    let mut active_model: ActiveModel = ft.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(recurrence) = payload.recurrence {
        active_model.recurrence = ActiveValue::Set(recurrence);
    }
    if let Some(description) = payload.description {
        active_model.description = ActiveValue::Set(Some(description));
    }
    if let Some(is_active) = payload.is_active {
        active_model.is_active = ActiveValue::Set(is_active);
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/fee-types/{id}
pub async fn delete_fee_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let ft = fee_types::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee type not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_type:delete", Some(ft.organization_id), None).await?;

    let active_model: ActiveModel = ft.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
