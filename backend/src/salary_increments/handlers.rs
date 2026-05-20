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
    entities::salary_increments::{self, ActiveModel},
    salary_increments::dto::{CreateSalaryIncrementRequest, SalaryIncrementResponse, UpdateSalaryIncrementRequest},
};

fn to_response(m: salary_increments::Model) -> SalaryIncrementResponse {
    SalaryIncrementResponse {
        id: m.id,
        staff_id: m.staff_id,
        staff_salary_id: m.staff_salary_id,
        increment_type: m.increment_type,
        previous_basic_salary: m.previous_basic_salary,
        new_basic_salary: m.new_basic_salary,
        increment_amount: m.increment_amount,
        effective_from: m.effective_from.to_string(),
        reason: m.reason,
        approved_by: m.approved_by,
    }
}

pub async fn create_salary_increment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateSalaryIncrementRequest>,
) -> Result<Json<SalaryIncrementResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_increment:create", None, None).await?;

    let effective_from = NaiveDate::parse_from_str(&payload.effective_from, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid effective_from: {}", e)))?;

    let record = ActiveModel {
        staff_id: ActiveValue::Set(payload.staff_id),
        staff_salary_id: ActiveValue::Set(payload.staff_salary_id),
        increment_type: ActiveValue::Set(payload.increment_type),
        previous_basic_salary: ActiveValue::Set(payload.previous_basic_salary),
        new_basic_salary: ActiveValue::Set(payload.new_basic_salary),
        increment_amount: ActiveValue::Set(payload.increment_amount),
        effective_from: ActiveValue::Set(effective_from),
        reason: ActiveValue::Set(payload.reason),
        approved_by: ActiveValue::Set(payload.approved_by),
        created_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_salary_increments(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<SalaryIncrementResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_increment:read", None, None).await?;

    let records = salary_increments::Entity::find()
        .all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_salary_increment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<SalaryIncrementResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_increment:read", None, None).await?;

    let record = salary_increments::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Salary increment not found".to_string()))?;

    Ok(Json(to_response(record)))
}

pub async fn update_salary_increment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateSalaryIncrementRequest>,
) -> Result<Json<SalaryIncrementResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_increment:update", None, None).await?;

    let record = salary_increments::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Salary increment not found".to_string()))?;

    let mut active: ActiveModel = record.into();

    if let Some(reason) = payload.reason { active.reason = ActiveValue::Set(Some(reason)); }

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_salary_increment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_increment:delete", None, None).await?;

    let record = salary_increments::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Salary increment not found".to_string()))?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
