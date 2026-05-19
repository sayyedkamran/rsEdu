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
    entities::master_classes::{self, ActiveModel},
    master_classes::dto::{CreateMasterClassRequest, MasterClassResponse, UpdateMasterClassRequest},
};

fn to_response(mc: master_classes::Model) -> MasterClassResponse {
    MasterClassResponse {
        id: mc.id,
        organization_id: mc.organization_id,
        stream_id: mc.stream_id,
        class_level_id: mc.class_level_id,
        name: mc.name,
        name_urdu: mc.name_urdu,
        order: mc.order,
        is_active: mc.is_active,
    }
}

// POST /api/v1/master-classes
pub async fn create_master_class(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateMasterClassRequest>,
) -> Result<Json<MasterClassResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "master_class:create", Some(payload.organization_id), None).await?;

    let new_mc = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        stream_id: ActiveValue::Set(payload.stream_id),
        class_level_id: ActiveValue::Set(payload.class_level_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        order: ActiveValue::Set(payload.order),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let mc = new_mc
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(mc)))
}

// GET /api/v1/master-classes
pub async fn get_master_classes(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<MasterClassResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "master_class:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = master_classes::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(master_classes::Column::OrganizationId.eq(org_id));
    }

    let master_classes = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(master_classes.into_iter().map(to_response).collect()))
}

// GET /api/v1/master-classes/{id}
pub async fn get_master_class(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<MasterClassResponse>, (StatusCode, String)> {
    let mc = master_classes::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Master class not found".to_string()))?;

    authorize(&*state.db, &claims, "master_class:read", Some(mc.organization_id), None).await?;

    Ok(Json(to_response(mc)))
}

// PUT /api/v1/master-classes/{id}
pub async fn update_master_class(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateMasterClassRequest>,
) -> Result<Json<MasterClassResponse>, (StatusCode, String)> {
    let mc = master_classes::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Master class not found".to_string()))?;

    authorize(&*state.db, &claims, "master_class:update", Some(mc.organization_id), None).await?;

    let mut active_model: ActiveModel = mc.into();

    if let Some(stream_id) = payload.stream_id {
        active_model.stream_id = ActiveValue::Set(stream_id);
    }
    if let Some(class_level_id) = payload.class_level_id {
        active_model.class_level_id = ActiveValue::Set(Some(class_level_id));
    }
    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(order) = payload.order {
        active_model.order = ActiveValue::Set(order);
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

// DELETE /api/v1/master-classes/{id}
pub async fn delete_master_class(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mc = master_classes::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Master class not found".to_string()))?;

    authorize(&*state.db, &claims, "master_class:delete", Some(mc.organization_id), None).await?;

    let active_model: ActiveModel = mc.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}