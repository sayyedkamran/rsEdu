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
    entities::streams::{self, ActiveModel},
    streams::dto::{CreateStreamRequest, StreamResponse, UpdateStreamRequest},
};

fn to_response(stream: streams::Model) -> StreamResponse {
    StreamResponse {
        id: stream.id,
        organization_id: stream.organization_id,
        name: stream.name,
        name_urdu: stream.name_urdu,
        description: stream.description,
        is_active: stream.is_active,
    }
}

// POST /api/v1/streams
pub async fn create_stream(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStreamRequest>,
) -> Result<Json<StreamResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "stream:create", Some(payload.organization_id), None).await?;

    let new_stream = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let stream = new_stream
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(stream)))
}

// GET /api/v1/streams
pub async fn get_streams(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StreamResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "stream:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = streams::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(streams::Column::OrganizationId.eq(org_id));
    }

    let streams = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(streams.into_iter().map(to_response).collect()))
}

// GET /api/v1/streams/{id}
pub async fn get_stream(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StreamResponse>, (StatusCode, String)> {
    let stream = streams::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Stream not found".to_string()))?;

    authorize(&*state.db, &claims, "stream:read", Some(stream.organization_id), None).await?;

    Ok(Json(to_response(stream)))
}

// PUT /api/v1/streams/{id}
pub async fn update_stream(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStreamRequest>,
) -> Result<Json<StreamResponse>, (StatusCode, String)> {
    let stream = streams::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Stream not found".to_string()))?;

    authorize(&*state.db, &claims, "stream:update", Some(stream.organization_id), None).await?;

    let mut active_model: ActiveModel = stream.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
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

// DELETE /api/v1/streams/{id}
pub async fn delete_stream(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let stream = streams::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Stream not found".to_string()))?;

    authorize(&*state.db, &claims, "stream:delete", Some(stream.organization_id), None).await?;

    let active_model: ActiveModel = stream.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}