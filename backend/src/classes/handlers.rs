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
    entities::classes::{self, ActiveModel},
    classes::dto::{CreateClassRequest, ClassResponse, UpdateClassRequest},
};

fn to_response(c: classes::Model) -> ClassResponse {
    ClassResponse {
        id: c.id,
        branch_id: c.branch_id,
        master_class_id: c.master_class_id,
        master_section_id: c.master_section_id,
        academic_year_id: c.academic_year_id,
        class_staff_id: c.class_staff_id,
        capacity: c.capacity,
        is_active: c.is_active,
    }
}

// POST /api/v1/classes
pub async fn create_class(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateClassRequest>,
) -> Result<Json<ClassResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "class:create", None, None).await?;

    let new_class = ActiveModel {
        branch_id: ActiveValue::Set(payload.branch_id),
        master_class_id: ActiveValue::Set(payload.master_class_id),
        master_section_id: ActiveValue::Set(payload.master_section_id),
        academic_year_id: ActiveValue::Set(payload.academic_year_id),
        class_staff_id: ActiveValue::Set(payload.class_staff_id),
        capacity: ActiveValue::Set(payload.capacity),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let c = new_class
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(c)))
}

// GET /api/v1/classes
pub async fn get_classes(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ClassResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "class:read", None, None).await?;

    let classes = classes::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(classes.into_iter().map(to_response).collect()))
}

// GET /api/v1/classes/{id}
pub async fn get_class(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ClassResponse>, (StatusCode, String)> {
    let c = classes::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class not found".to_string()))?;

    authorize(&*state.db, &claims, "class:read", None, None).await?;

    Ok(Json(to_response(c)))
}

// PUT /api/v1/classes/{id}
pub async fn update_class(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateClassRequest>,
) -> Result<Json<ClassResponse>, (StatusCode, String)> {
    let c = classes::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class not found".to_string()))?;

    authorize(&*state.db, &claims, "class:update", None, None).await?;

    let mut active_model: ActiveModel = c.into();

    if let Some(master_class_id) = payload.master_class_id {
        active_model.master_class_id = ActiveValue::Set(master_class_id);
    }
    if let Some(master_section_id) = payload.master_section_id {
        active_model.master_section_id = ActiveValue::Set(master_section_id);
    }
    if let Some(academic_year_id) = payload.academic_year_id {
        active_model.academic_year_id = ActiveValue::Set(academic_year_id);
    }
    if let Some(class_staff_id) = payload.class_staff_id {
        active_model.class_staff_id = ActiveValue::Set(Some(class_staff_id));
    }
    if let Some(capacity) = payload.capacity {
        active_model.capacity = ActiveValue::Set(Some(capacity));
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

// DELETE /api/v1/classes/{id}
pub async fn delete_class(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let c = classes::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class not found".to_string()))?;

    authorize(&*state.db, &claims, "class:delete", None, None).await?;

    let active_model: ActiveModel = c.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
