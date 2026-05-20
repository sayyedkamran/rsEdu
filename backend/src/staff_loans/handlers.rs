use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use chrono::Utc;

use crate::{
    AppState,
    auth::{utils::Claims, permissions::authorize, permissions::get_data_scope},
    entities::staff_loans::{self, ActiveModel},
    staff_loans::dto::{CreateStaffLoanRequest, StaffLoanResponse, UpdateStaffLoanRequest},
};

fn to_response(m: staff_loans::Model) -> StaffLoanResponse {
    StaffLoanResponse {
        id: m.id,
        staff_id: m.staff_id,
        branch_id: m.branch_id,
        amount: m.amount,
        purpose: m.purpose,
        installments: m.installments,
        installment_amount: m.installment_amount,
        start_month: m.start_month,
        start_year: m.start_year,
        status: m.status,
        approved_by: m.approved_by,
    }
}

pub async fn create_staff_loan(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStaffLoanRequest>,
) -> Result<Json<StaffLoanResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_loan:create", None, None).await?;

    let record = ActiveModel {
        staff_id: ActiveValue::Set(payload.staff_id),
        branch_id: ActiveValue::Set(payload.branch_id),
        amount: ActiveValue::Set(payload.amount),
        purpose: ActiveValue::Set(payload.purpose),
        installments: ActiveValue::Set(payload.installments),
        installment_amount: ActiveValue::Set(payload.installment_amount),
        start_month: ActiveValue::Set(payload.start_month),
        start_year: ActiveValue::Set(payload.start_year),
        status: ActiveValue::Set("active".to_string()),
        approved_by: ActiveValue::Set(payload.approved_by),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_staff_loans(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StaffLoanResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_loan:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = staff_loans::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(staff_loans::Column::BranchId.eq(branch_id));
    }

    let records = query.all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_staff_loan(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StaffLoanResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_loan:read", None, None).await?;

    let record = staff_loans::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff loan not found".to_string()))?;

    Ok(Json(to_response(record)))
}

pub async fn update_staff_loan(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStaffLoanRequest>,
) -> Result<Json<StaffLoanResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_loan:update", None, None).await?;

    let record = staff_loans::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff loan not found".to_string()))?;

    let mut active: ActiveModel = record.into();

    if let Some(purpose) = payload.purpose { active.purpose = ActiveValue::Set(Some(purpose)); }
    if let Some(status) = payload.status { active.status = ActiveValue::Set(status); }
    active.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_staff_loan(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff_loan:delete", None, None).await?;

    let record = staff_loans::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff loan not found".to_string()))?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
