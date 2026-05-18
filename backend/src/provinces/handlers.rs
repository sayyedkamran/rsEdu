use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use chrono::Utc;

use crate::{
    AppState,
    entities::provinces::{self, ActiveModel},
    provinces::dto::{CreateProvinceRequest, ProvinceResponse, UpdateProvinceRequest},
};

fn to_response(province: provinces::Model) -> ProvinceResponse {
    ProvinceResponse {
        id: province.id,
        name: province.name,
        name_urdu: province.name_urdu,
        code: province.code,
        is_active: province.is_active,
    }
}

// POST /api/v1/provinces
pub async fn create_province(
    State(state): State<AppState>,
    Json(payload): Json<CreateProvinceRequest>,
) -> Result<Json<ProvinceResponse>, (StatusCode, String)> {
    let new_province = ActiveModel {
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        code: ActiveValue::Set(payload.code),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let province = new_province
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(province)))
}

// GET /api/v1/provinces
pub async fn get_provinces(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProvinceResponse>>, (StatusCode, String)> {
    let provinces = provinces::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(provinces.into_iter().map(to_response).collect()))
}

// GET /api/v1/provinces/{id}
pub async fn get_province(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ProvinceResponse>, (StatusCode, String)> {
    let province = provinces::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Province not found".to_string()))?;

    Ok(Json(to_response(province)))
}

// PUT /api/v1/provinces/{id}
pub async fn update_province(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateProvinceRequest>,
) -> Result<Json<ProvinceResponse>, (StatusCode, String)> {
    let province = provinces::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Province not found".to_string()))?;

    let mut active_model: ActiveModel = province.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(code) = payload.code {
        active_model.code = ActiveValue::Set(code);
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

// DELETE /api/v1/provinces/{id}
pub async fn delete_province(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let province = provinces::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Province not found".to_string()))?;

    let active_model: ActiveModel = province.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}