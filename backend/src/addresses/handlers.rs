use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use chrono::Utc;

use crate::{
    AppState,
    auth::{utils::Claims, permissions::authorize},
    entities::addresses::{self, ActiveModel},
    addresses::dto::{CreateAddressRequest, AddressResponse, UpdateAddressRequest},
};

fn to_response(a: addresses::Model) -> AddressResponse {
    AddressResponse {
        id: a.id,
        entity_type: a.entity_type,
        entity_id: a.entity_id,
        address_type: a.address_type,
        address_line: a.address_line,
        area: a.area,
        city_id: a.city_id,
        postal_code: a.postal_code,
        is_primary: a.is_primary,
        is_active: a.is_active,
    }
}

// POST /api/v1/addresses
pub async fn create_address(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateAddressRequest>,
) -> Result<Json<AddressResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "address:create", None, None).await?;

    let new_address = ActiveModel {
        entity_type: ActiveValue::Set(payload.entity_type),
        entity_id: ActiveValue::Set(payload.entity_id),
        address_type: ActiveValue::Set(payload.address_type),
        address_line: ActiveValue::Set(payload.address_line),
        area: ActiveValue::Set(payload.area),
        city_id: ActiveValue::Set(payload.city_id),
        postal_code: ActiveValue::Set(payload.postal_code),
        is_primary: ActiveValue::Set(payload.is_primary),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let address = new_address
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(address)))
}

// GET /api/v1/addresses
pub async fn get_addresses(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<AddressResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "address:read", None, None).await?;

    let records = addresses::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/addresses/{id}
pub async fn get_address(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<AddressResponse>, (StatusCode, String)> {
    let address = addresses::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Address not found".to_string()))?;

    authorize(&*state.db, &claims, "address:read", None, None).await?;

    Ok(Json(to_response(address)))
}

// PUT /api/v1/addresses/{id}
pub async fn update_address(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateAddressRequest>,
) -> Result<Json<AddressResponse>, (StatusCode, String)> {
    let address = addresses::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Address not found".to_string()))?;

    authorize(&*state.db, &claims, "address:update", None, None).await?;

    let mut active_model: ActiveModel = address.into();

    if let Some(address_type) = payload.address_type {
        active_model.address_type = ActiveValue::Set(address_type);
    }
    if let Some(address_line) = payload.address_line {
        active_model.address_line = ActiveValue::Set(address_line);
    }
    if let Some(area) = payload.area {
        active_model.area = ActiveValue::Set(Some(area));
    }
    if let Some(city_id) = payload.city_id {
        active_model.city_id = ActiveValue::Set(Some(city_id));
    }
    if let Some(postal_code) = payload.postal_code {
        active_model.postal_code = ActiveValue::Set(Some(postal_code));
    }
    if let Some(is_primary) = payload.is_primary {
        active_model.is_primary = ActiveValue::Set(is_primary);
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

// DELETE /api/v1/addresses/{id}
pub async fn delete_address(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let address = addresses::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Address not found".to_string()))?;

    authorize(&*state.db, &claims, "address:delete", None, None).await?;

    let active_model: ActiveModel = address.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
