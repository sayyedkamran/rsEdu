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
    entities::class_progressions::{self, ActiveModel},
    class_progressions::dto::{CreateClassProgressionRequest, ClassProgressionResponse, UpdateClassProgressionRequest},
};

fn to_response(cp: class_progressions::Model) -> ClassProgressionResponse {
    ClassProgressionResponse {
        id: cp.id,
        from_class_id: cp.from_class_id,
        to_class_id: cp.to_class_id,
    }
}

// POST /api/v1/class-progressions
pub async fn create_class_progression(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateClassProgressionRequest>,
) -> Result<Json<ClassProgressionResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "class_progression:create", None, None).await?;

    let new_cp = ActiveModel {
        from_class_id: ActiveValue::Set(payload.from_class_id),
        to_class_id: ActiveValue::Set(payload.to_class_id),
        created_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let cp = new_cp
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(cp)))
}

// GET /api/v1/class-progressions
pub async fn get_class_progressions(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ClassProgressionResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "class_progression:read", None, None).await?;

    let records = class_progressions::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/class-progressions/{id}
pub async fn get_class_progression(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ClassProgressionResponse>, (StatusCode, String)> {
    let cp = class_progressions::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class progression not found".to_string()))?;

    authorize(&*state.db, &claims, "class_progression:read", None, None).await?;

    Ok(Json(to_response(cp)))
}

// PUT /api/v1/class-progressions/{id}
pub async fn update_class_progression(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateClassProgressionRequest>,
) -> Result<Json<ClassProgressionResponse>, (StatusCode, String)> {
    let cp = class_progressions::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class progression not found".to_string()))?;

    authorize(&*state.db, &claims, "class_progression:update", None, None).await?;

    let mut active_model: ActiveModel = cp.into();

    if let Some(from_class_id) = payload.from_class_id {
        active_model.from_class_id = ActiveValue::Set(from_class_id);
    }
    if let Some(to_class_id) = payload.to_class_id {
        active_model.to_class_id = ActiveValue::Set(to_class_id);
    }

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/class-progressions/{id}
pub async fn delete_class_progression(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let cp = class_progressions::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class progression not found".to_string()))?;

    authorize(&*state.db, &claims, "class_progression:delete", None, None).await?;

    let active_model: ActiveModel = cp.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
