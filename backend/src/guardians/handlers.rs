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
    entities::guardians::{self, ActiveModel},
    guardians::dto::{CreateGuardianRequest, GuardianResponse, UpdateGuardianRequest},
};

fn to_response(g: guardians::Model) -> GuardianResponse {
    GuardianResponse {
        id: g.id,
        user_id: g.user_id,
        first_name: g.first_name,
        last_name: g.last_name,
        relationship: g.relationship,
        cnic: g.cnic,
        occupation: g.occupation,
        employer: g.employer,
        is_active: g.is_active,
    }
}

// POST /api/v1/guardians
pub async fn create_guardian(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateGuardianRequest>,
) -> Result<Json<GuardianResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "guardian:create", None, None).await?;

    let new_guardian = ActiveModel {
        user_id: ActiveValue::Set(payload.user_id),
        first_name: ActiveValue::Set(payload.first_name),
        last_name: ActiveValue::Set(payload.last_name),
        relationship: ActiveValue::Set(payload.relationship),
        cnic: ActiveValue::Set(payload.cnic),
        occupation: ActiveValue::Set(payload.occupation),
        employer: ActiveValue::Set(payload.employer),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let g = new_guardian
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(g)))
}

// GET /api/v1/guardians
pub async fn get_guardians(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<GuardianResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "guardian:read", None, None).await?;

    let guardians = guardians::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(guardians.into_iter().map(to_response).collect()))
}

// GET /api/v1/guardians/{id}
pub async fn get_guardian(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<GuardianResponse>, (StatusCode, String)> {
    let g = guardians::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guardian not found".to_string()))?;

    authorize(&*state.db, &claims, "guardian:read", None, None).await?;

    Ok(Json(to_response(g)))
}

// PUT /api/v1/guardians/{id}
pub async fn update_guardian(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateGuardianRequest>,
) -> Result<Json<GuardianResponse>, (StatusCode, String)> {
    let g = guardians::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guardian not found".to_string()))?;

    authorize(&*state.db, &claims, "guardian:update", None, None).await?;

    let mut active_model: ActiveModel = g.into();

    if let Some(first_name) = payload.first_name {
        active_model.first_name = ActiveValue::Set(first_name);
    }
    if let Some(last_name) = payload.last_name {
        active_model.last_name = ActiveValue::Set(last_name);
    }
    if let Some(relationship) = payload.relationship {
        active_model.relationship = ActiveValue::Set(relationship);
    }
    if let Some(cnic) = payload.cnic {
        active_model.cnic = ActiveValue::Set(Some(cnic));
    }
    if let Some(occupation) = payload.occupation {
        active_model.occupation = ActiveValue::Set(Some(occupation));
    }
    if let Some(employer) = payload.employer {
        active_model.employer = ActiveValue::Set(Some(employer));
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

// DELETE /api/v1/guardians/{id}
pub async fn delete_guardian(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let g = guardians::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guardian not found".to_string()))?;

    authorize(&*state.db, &claims, "guardian:delete", None, None).await?;

    let active_model: ActiveModel = g.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
