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
    entities::staff_monthly_salaries::{self, ActiveModel},
    staff_monthly_salaries::dto::{CreateStaffMonthlySalaryRequest, StaffMonthlySalaryResponse, UpdateStaffMonthlySalaryRequest},
};

fn to_response(m: staff_monthly_salaries::Model) -> StaffMonthlySalaryResponse {
    StaffMonthlySalaryResponse {
        id: m.id,
        staff_id: m.staff_id,
        branch_id: m.branch_id,
        academic_year_id: m.academic_year_id,
        salary_month: m.salary_month,
        salary_year: m.salary_year,
        working_days: m.working_days,
        present_days: m.present_days,
        absent_days: m.absent_days,
        gross_salary: m.gross_salary,
        total_deductions: m.total_deductions,
        net_salary: m.net_salary,
        status: m.status,
        approved_by: m.approved_by,
        payment_date: m.payment_date.map(|d| d.to_string()),
        payment_method_id: m.payment_method_id,
        remarks: m.remarks,
        generated_by: m.generated_by,
    }
}

pub async fn create_staff_monthly_salary(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStaffMonthlySalaryRequest>,
) -> Result<Json<StaffMonthlySalaryResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_monthly_salary:create", None, None).await?;

    let record = ActiveModel {
        staff_id: ActiveValue::Set(payload.staff_id),
        branch_id: ActiveValue::Set(payload.branch_id),
        academic_year_id: ActiveValue::Set(payload.academic_year_id),
        salary_month: ActiveValue::Set(payload.salary_month),
        salary_year: ActiveValue::Set(payload.salary_year),
        working_days: ActiveValue::Set(payload.working_days),
        present_days: ActiveValue::Set(payload.present_days),
        absent_days: ActiveValue::Set(payload.absent_days),
        gross_salary: ActiveValue::Set(payload.gross_salary),
        total_deductions: ActiveValue::Set(payload.total_deductions),
        net_salary: ActiveValue::Set(payload.net_salary),
        status: ActiveValue::Set("draft".to_string()),
        approved_by: ActiveValue::Set(None),
        approved_at: ActiveValue::Set(None),
        payment_date: ActiveValue::Set(None),
        payment_method_id: ActiveValue::Set(None),
        remarks: ActiveValue::Set(payload.remarks),
        generated_by: ActiveValue::Set(payload.generated_by),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_staff_monthly_salaries(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StaffMonthlySalaryResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_monthly_salary:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = staff_monthly_salaries::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(staff_monthly_salaries::Column::BranchId.eq(branch_id));
    }

    let records = query.all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_staff_monthly_salary(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StaffMonthlySalaryResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_monthly_salary:read", None, None).await?;

    let record = staff_monthly_salaries::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff monthly salary not found".to_string()))?;

    Ok(Json(to_response(record)))
}

pub async fn update_staff_monthly_salary(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStaffMonthlySalaryRequest>,
) -> Result<Json<StaffMonthlySalaryResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_monthly_salary:update", None, None).await?;

    let record = staff_monthly_salaries::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff monthly salary not found".to_string()))?;

    let mut active: ActiveModel = record.into();

    if let Some(v) = payload.working_days { active.working_days = ActiveValue::Set(v); }
    if let Some(v) = payload.present_days { active.present_days = ActiveValue::Set(v); }
    if let Some(v) = payload.absent_days { active.absent_days = ActiveValue::Set(v); }
    if let Some(v) = payload.gross_salary { active.gross_salary = ActiveValue::Set(v); }
    if let Some(v) = payload.total_deductions { active.total_deductions = ActiveValue::Set(v); }
    if let Some(v) = payload.net_salary { active.net_salary = ActiveValue::Set(v); }
    if let Some(status) = payload.status { active.status = ActiveValue::Set(status); }
    if let Some(approved_by) = payload.approved_by {
        active.approved_by = ActiveValue::Set(Some(approved_by));
        active.approved_at = ActiveValue::Set(Some(Utc::now().into()));
    }
    if let Some(pd) = payload.payment_date {
        let d = NaiveDate::parse_from_str(&pd, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid payment_date: {}", e)))?;
        active.payment_date = ActiveValue::Set(Some(d));
    }
    if let Some(v) = payload.payment_method_id { active.payment_method_id = ActiveValue::Set(Some(v)); }
    if let Some(remarks) = payload.remarks { active.remarks = ActiveValue::Set(Some(remarks)); }
    active.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_staff_monthly_salary(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_monthly_salary:delete", None, None).await?;

    let record = staff_monthly_salaries::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff monthly salary not found".to_string()))?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
