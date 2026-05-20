use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use chrono::{NaiveDate, Utc};

use crate::{
    AppState,
    auth::{utils::Claims, permissions::authorize, permissions::get_data_scope},
    entities::staff_salaries::{self, ActiveModel},
    staff_salaries::dto::{CreateStaffSalaryRequest, StaffSalaryResponse, UpdateStaffSalaryRequest},
};

fn to_response(m: staff_salaries::Model) -> StaffSalaryResponse {
    StaffSalaryResponse {
        id: m.id,
        staff_id: m.staff_id,
        salary_structure_id: m.salary_structure_id,
        basic_salary: m.basic_salary,
        effective_from: m.effective_from.to_string(),
        effective_to: m.effective_to.map(|d| d.to_string()),
        remarks: m.remarks,
        assigned_by: m.assigned_by,
        is_active: m.is_active,
    }
}

pub async fn create_staff_salary(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStaffSalaryRequest>,
) -> Result<Json<StaffSalaryResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_salary:create", None, None).await?;

    let effective_from = NaiveDate::parse_from_str(&payload.effective_from, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid effective_from: {}", e)))?;

    let effective_to = payload.effective_to.as_deref()
        .map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid effective_to: {}", e)))?;

    let record = ActiveModel {
        staff_id: ActiveValue::Set(payload.staff_id),
        salary_structure_id: ActiveValue::Set(payload.salary_structure_id),
        basic_salary: ActiveValue::Set(payload.basic_salary),
        effective_from: ActiveValue::Set(effective_from),
        effective_to: ActiveValue::Set(effective_to),
        remarks: ActiveValue::Set(payload.remarks),
        assigned_by: ActiveValue::Set(payload.assigned_by),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_staff_salaries(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StaffSalaryResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_salary:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = staff_salaries::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(staff_salaries::Column::StaffId.eq(branch_id));
    }

    let records = query.all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_staff_salary(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StaffSalaryResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_salary:read", None, None).await?;

    let record = staff_salaries::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff salary not found".to_string()))?;

    Ok(Json(to_response(record)))
}

pub async fn update_staff_salary(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStaffSalaryRequest>,
) -> Result<Json<StaffSalaryResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_salary:update", None, None).await?;

    let record = staff_salaries::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff salary not found".to_string()))?;

    let mut active: ActiveModel = record.into();

    if let Some(basic_salary) = payload.basic_salary { active.basic_salary = ActiveValue::Set(basic_salary); }
    if let Some(eff_to) = payload.effective_to {
        let d = NaiveDate::parse_from_str(&eff_to, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid effective_to: {}", e)))?;
        active.effective_to = ActiveValue::Set(Some(d));
    }
    if let Some(remarks) = payload.remarks { active.remarks = ActiveValue::Set(Some(remarks)); }
    if let Some(is_active) = payload.is_active { active.is_active = ActiveValue::Set(is_active); }
    active.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_staff_salary(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_salary:delete", None, None).await?;

    let record = staff_salaries::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff salary not found".to_string()))?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
