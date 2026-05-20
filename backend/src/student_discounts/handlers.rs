use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use chrono::{NaiveDate, Utc};

use crate::{
    AppState,
    auth::{utils::Claims, permissions::authorize},
    entities::student_discounts::{self, ActiveModel},
    student_discounts::dto::{CreateStudentDiscountRequest, StudentDiscountResponse, UpdateStudentDiscountRequest},
};

fn to_response(sd: student_discounts::Model) -> StudentDiscountResponse {
    StudentDiscountResponse {
        id: sd.id,
        student_id: sd.student_id,
        discount_id: sd.discount_id,
        academic_year_id: sd.academic_year_id,
        fee_type_id: sd.fee_type_id,
        discount_type: sd.discount_type,
        value: sd.value,
        start_date: sd.start_date.map(|d| d.to_string()),
        end_date: sd.end_date.map(|d| d.to_string()),
        reason: sd.reason,
        status: sd.status,
        requested_by: sd.requested_by,
        approved_by: sd.approved_by,
        rejection_reason: sd.rejection_reason,
        is_active: sd.is_active,
    }
}

// POST /api/v1/student-discounts
pub async fn create_student_discount(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStudentDiscountRequest>,
) -> Result<Json<StudentDiscountResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "student_discount:create", None, None).await?;

    let start_date = payload.start_date
        .as_deref()
        .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid start_date: {}", e)))?;

    let end_date = payload.end_date
        .as_deref()
        .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid end_date: {}", e)))?;

    let new_sd = ActiveModel {
        student_id: ActiveValue::Set(payload.student_id),
        discount_id: ActiveValue::Set(payload.discount_id),
        academic_year_id: ActiveValue::Set(payload.academic_year_id),
        fee_type_id: ActiveValue::Set(payload.fee_type_id),
        discount_type: ActiveValue::Set(payload.discount_type),
        value: ActiveValue::Set(payload.value),
        start_date: ActiveValue::Set(start_date),
        end_date: ActiveValue::Set(end_date),
        reason: ActiveValue::Set(payload.reason),
        status: ActiveValue::Set(payload.status),
        requested_by: ActiveValue::Set(payload.requested_by),
        approved_by: ActiveValue::Set(None),
        rejection_reason: ActiveValue::Set(None),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let sd = new_sd
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(sd)))
}

// GET /api/v1/student-discounts
pub async fn get_student_discounts(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StudentDiscountResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "student_discount:read", None, None).await?;

    let records = student_discounts::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/student-discounts/{id}
pub async fn get_student_discount(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StudentDiscountResponse>, (StatusCode, String)> {
    let sd = student_discounts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student discount not found".to_string()))?;

    authorize(&*state.db, &claims, "student_discount:read", None, None).await?;

    Ok(Json(to_response(sd)))
}

// PUT /api/v1/student-discounts/{id}
pub async fn update_student_discount(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStudentDiscountRequest>,
) -> Result<Json<StudentDiscountResponse>, (StatusCode, String)> {
    let sd = student_discounts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student discount not found".to_string()))?;

    authorize(&*state.db, &claims, "student_discount:update", None, None).await?;

    let mut active_model: ActiveModel = sd.into();

    if let Some(discount_type) = payload.discount_type {
        active_model.discount_type = ActiveValue::Set(discount_type);
    }
    if let Some(value) = payload.value {
        active_model.value = ActiveValue::Set(value);
    }
    if let Some(start_date_str) = payload.start_date {
        let date = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid start_date: {}", e)))?;
        active_model.start_date = ActiveValue::Set(Some(date));
    }
    if let Some(end_date_str) = payload.end_date {
        let date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid end_date: {}", e)))?;
        active_model.end_date = ActiveValue::Set(Some(date));
    }
    if let Some(reason) = payload.reason {
        active_model.reason = ActiveValue::Set(Some(reason));
    }
    if let Some(status) = payload.status {
        active_model.status = ActiveValue::Set(status);
    }
    if let Some(approved_by) = payload.approved_by {
        active_model.approved_by = ActiveValue::Set(Some(approved_by));
    }
    if let Some(rejection_reason) = payload.rejection_reason {
        active_model.rejection_reason = ActiveValue::Set(Some(rejection_reason));
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

// DELETE /api/v1/student-discounts/{id}
pub async fn delete_student_discount(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let sd = student_discounts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Student discount not found".to_string()))?;

    authorize(&*state.db, &claims, "student_discount:delete", None, None).await?;

    let active_model: ActiveModel = sd.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
