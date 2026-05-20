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
    entities::allowance_deduction_types::{self, ActiveModel},
    allowance_deduction_types::dto::{CreateAllowanceDeductionTypeRequest, AllowanceDeductionTypeResponse, UpdateAllowanceDeductionTypeRequest},
};

fn to_response(m: allowance_deduction_types::Model) -> AllowanceDeductionTypeResponse {
    AllowanceDeductionTypeResponse {
        id: m.id,
        organization_id: m.organization_id,
        branch_id: m.branch_id,
        name: m.name,
        name_urdu: m.name_urdu,
        r#type: m.r#type,
        occurrence: m.occurrence,
        frequency: m.frequency,
        is_taxable: m.is_taxable,
        description: m.description,
        is_active: m.is_active,
    }
}

pub async fn create_allowance_deduction_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateAllowanceDeductionTypeRequest>,
) -> Result<Json<AllowanceDeductionTypeResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "allowance_deduction_type:create", Some(payload.organization_id), None).await?;

    let record = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        branch_id: ActiveValue::Set(payload.branch_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        r#type: ActiveValue::Set(payload.r#type),
        occurrence: ActiveValue::Set(payload.occurrence),
        frequency: ActiveValue::Set(payload.frequency),
        is_taxable: ActiveValue::Set(payload.is_taxable),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let result = record.insert(&*state.db).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(result)))
}

pub async fn get_allowance_deduction_types(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<AllowanceDeductionTypeResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "allowance_deduction_type:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = allowance_deduction_types::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(allowance_deduction_types::Column::OrganizationId.eq(org_id));
    }

    let records = query.all(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

pub async fn get_allowance_deduction_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<AllowanceDeductionTypeResponse>, (StatusCode, String)> {
    let record = allowance_deduction_types::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Allowance/deduction type not found".to_string()))?;

    authorize(&*state.db, &claims, "allowance_deduction_type:read", Some(record.organization_id), None).await?;

    Ok(Json(to_response(record)))
}

pub async fn update_allowance_deduction_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateAllowanceDeductionTypeRequest>,
) -> Result<Json<AllowanceDeductionTypeResponse>, (StatusCode, String)> {
    let record = allowance_deduction_types::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Allowance/deduction type not found".to_string()))?;

    authorize(&*state.db, &claims, "allowance_deduction_type:update", Some(record.organization_id), None).await?;

    let mut active: ActiveModel = record.into();

    if let Some(name) = payload.name { active.name = ActiveValue::Set(name); }
    if let Some(name_urdu) = payload.name_urdu { active.name_urdu = ActiveValue::Set(Some(name_urdu)); }
    if let Some(t) = payload.r#type { active.r#type = ActiveValue::Set(t); }
    if let Some(occurrence) = payload.occurrence { active.occurrence = ActiveValue::Set(occurrence); }
    if let Some(frequency) = payload.frequency { active.frequency = ActiveValue::Set(Some(frequency)); }
    if let Some(is_taxable) = payload.is_taxable { active.is_taxable = ActiveValue::Set(is_taxable); }
    if let Some(description) = payload.description { active.description = ActiveValue::Set(Some(description)); }
    if let Some(is_active) = payload.is_active { active.is_active = ActiveValue::Set(is_active); }
    active.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active.update(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

pub async fn delete_allowance_deduction_type(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let record = allowance_deduction_types::Entity::find_by_id(id)
        .one(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Allowance/deduction type not found".to_string()))?;

    authorize(&*state.db, &claims, "allowance_deduction_type:delete", Some(record.organization_id), None).await?;

    let active: ActiveModel = record.into();
    active.delete(&*state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
