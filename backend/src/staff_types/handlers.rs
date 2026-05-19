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
    entities::staff_types::{self, ActiveModel},
    staff_types::dto::{CreateStaffTypeRequest, StaffTypeResponse, UpdateStaffTypeRequest},
};

fn to_response(st: staff_types::Model) -> StaffTypeResponse {
    StaffTypeResponse {
        id: st.id,
        organization_id: st.organization_id,
        name: st.name,
        name_urdu: st.name_urdu,
        is_teaching: st.is_teaching,
        description: st.description,
        is_active: st.is_active,
    }
}

// POST /api/v1/staff-types
pub async fn create_staff_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStaffTypeRequest>,
) -> Result<Json<StaffTypeResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_type:create", Some(payload.organization_id), None).await?;

    let new_st = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        is_teaching: ActiveValue::Set(payload.is_teaching),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let st = new_st
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(st)))
}

// GET /api/v1/staff-types
pub async fn get_staff_types(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StaffTypeResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_type:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = staff_types::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(staff_types::Column::OrganizationId.eq(org_id));
    }

    let staff_types = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(staff_types.into_iter().map(to_response).collect()))
}

// GET /api/v1/staff-types/{id}
pub async fn get_staff_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StaffTypeResponse>, (StatusCode, String)> {
    let st = staff_types::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff type not found".to_string()))?;

    authorize(&*state.db, &claims, "staff_type:read", Some(st.organization_id), None).await?;

    Ok(Json(to_response(st)))
}

// PUT /api/v1/staff-types/{id}
pub async fn update_staff_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStaffTypeRequest>,
) -> Result<Json<StaffTypeResponse>, (StatusCode, String)> {
    let st = staff_types::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff type not found".to_string()))?;

    authorize(&*state.db, &claims, "staff_type:update", Some(st.organization_id), None).await?;

    let mut active_model: ActiveModel = st.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(is_teaching) = payload.is_teaching {
        active_model.is_teaching = ActiveValue::Set(is_teaching);
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

// DELETE /api/v1/staff-types/{id}
pub async fn delete_staff_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let st = staff_types::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff type not found".to_string()))?;

    authorize(&*state.db, &claims, "staff_type:delete", Some(st.organization_id), None).await?;

    let active_model: ActiveModel = st.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
