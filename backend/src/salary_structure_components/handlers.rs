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
    entities::salary_structure_components::{self, ActiveModel},
    salary_structure_components::dto::{CreateSalaryStructureComponentRequest, SalaryStructureComponentResponse, UpdateSalaryStructureComponentRequest},
};

fn to_response(m: salary_structure_components::Model) -> SalaryStructureComponentResponse {
    SalaryStructureComponentResponse {
        id: m.id,
        salary_structure_id: m.salary_structure_id,
        allowance_deduction_type_id: m.allowance_deduction_type_id,
        amount: m.amount,
        calculation_type: m.calculation_type,
        percentage_base: m.percentage_base,
        is_active: m.is_active,
    }
}

pub async fn create_salary_structure_component(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateSalaryStructureComponentRequest>,
) -> Result<Json<SalaryStructureComponentResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_structure_component:create", None, None).await?;

    let record = ActiveModel {
        salary_structure_id: ActiveValue::Set(payload.salary_structure_id),
        allowance_deduction_type_id: ActiveValue::Set(payload.allowance_deduction_type_id),
        amount: ActiveValue::Set(payload.amount),
        calculation_type: ActiveValue::Set(payload.calculation_type),
        percentage_base: ActiveValue::Set(payload.percentage_base),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_salary_structure_components(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<SalaryStructureComponentResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_structure_component:read", None, None).await?;

    let records = salary_structure_components::Entity::find()
        .all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_salary_structure_component(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<SalaryStructureComponentResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_structure_component:read", None, None).await?;

    let record = salary_structure_components::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Salary structure component not found".to_string()))?;

    Ok(Json(to_response(record)))
}

pub async fn update_salary_structure_component(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateSalaryStructureComponentRequest>,
) -> Result<Json<SalaryStructureComponentResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_structure_component:update", None, None).await?;

    let record = salary_structure_components::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Salary structure component not found".to_string()))?;

    let mut active: ActiveModel = record.into();

    if let Some(amount) = payload.amount { active.amount = ActiveValue::Set(amount); }
    if let Some(calc) = payload.calculation_type { active.calculation_type = ActiveValue::Set(calc); }
    if let Some(pct_base) = payload.percentage_base { active.percentage_base = ActiveValue::Set(Some(pct_base)); }
    if let Some(is_active) = payload.is_active { active.is_active = ActiveValue::Set(is_active); }
    active.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_salary_structure_component(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    authorize(&*state.db, &claims, "salary_structure_component:delete", None, None).await?;

    let record = salary_structure_components::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Salary structure component not found".to_string()))?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
