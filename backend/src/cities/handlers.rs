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
    entities::cities::{self, ActiveModel},
    cities::dto::{CreateCityRequest, CityResponse, UpdateCityRequest},
};

fn to_response(city: cities::Model) -> CityResponse {
    CityResponse {
        id: city.id,
        name: city.name,
        name_urdu: city.name_urdu,
        province_id: city.province_id,
        is_active: city.is_active,
    }
}

// POST /api/v1/cities
pub async fn create_city(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateCityRequest>,
) -> Result<Json<CityResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "city:create", None, None).await?;

    let new_city = ActiveModel {
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        province_id: ActiveValue::Set(payload.province_id),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let city = new_city
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(city)))
}

// GET /api/v1/cities
pub async fn get_cities(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<CityResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "city:read", None, None).await?;

    let cities = cities::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(cities.into_iter().map(to_response).collect()))
}

// GET /api/v1/cities/{id}
pub async fn get_city(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<CityResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "city:read", None, None).await?;

    let city = cities::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "City not found".to_string()))?;

    Ok(Json(to_response(city)))
}

// PUT /api/v1/cities/{id}
pub async fn update_city(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCityRequest>,
) -> Result<Json<CityResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "city:update", None, None).await?;

    let city = cities::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "City not found".to_string()))?;

    let mut active_model: ActiveModel = city.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
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

// DELETE /api/v1/cities/{id}
pub async fn delete_city(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    authorize(&*state.db, &claims, "city:delete", None, None).await?;

    let city = cities::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "City not found".to_string()))?;

    let active_model: ActiveModel = city.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}