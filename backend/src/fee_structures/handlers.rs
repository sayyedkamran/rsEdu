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
    entities::fee_structures::{self, ActiveModel},
    fee_structures::dto::{CreateFeeStructureRequest, FeeStructureResponse, UpdateFeeStructureRequest},
};

fn to_response(fs: fee_structures::Model) -> FeeStructureResponse {
    FeeStructureResponse {
        id: fs.id,
        organization_id: fs.organization_id,
        branch_id: fs.branch_id,
        class_level_id: fs.class_level_id,
        master_class_id: fs.master_class_id,
        student_id: fs.student_id,
        fee_type_id: fs.fee_type_id,
        academic_year_id: fs.academic_year_id,
        amount: fs.amount,
        due_day: fs.due_day,
        bill_generation_frequency: fs.bill_generation_frequency,
        late_fee_type: fs.late_fee_type,
        late_fee_value: fs.late_fee_value,
        is_active: fs.is_active,
    }
}

// POST /api/v1/fee-structures
pub async fn create_fee_structure(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateFeeStructureRequest>,
) -> Result<Json<FeeStructureResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "fee_structure:create", Some(payload.organization_id), None).await?;

    let new_fs = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        branch_id: ActiveValue::Set(payload.branch_id),
        class_level_id: ActiveValue::Set(payload.class_level_id),
        master_class_id: ActiveValue::Set(payload.master_class_id),
        student_id: ActiveValue::Set(payload.student_id),
        fee_type_id: ActiveValue::Set(payload.fee_type_id),
        academic_year_id: ActiveValue::Set(payload.academic_year_id),
        amount: ActiveValue::Set(payload.amount),
        due_day: ActiveValue::Set(payload.due_day),
        bill_generation_frequency: ActiveValue::Set(payload.bill_generation_frequency),
        late_fee_type: ActiveValue::Set(payload.late_fee_type),
        late_fee_value: ActiveValue::Set(payload.late_fee_value),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let fs = new_fs
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(fs)))
}

// GET /api/v1/fee-structures
pub async fn get_fee_structures(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<FeeStructureResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "fee_structure:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = fee_structures::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(fee_structures::Column::OrganizationId.eq(org_id));
    }

    let fee_structures = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(fee_structures.into_iter().map(to_response).collect()))
}

// GET /api/v1/fee-structures/{id}
pub async fn get_fee_structure(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<FeeStructureResponse>, (StatusCode, String)> {
    let fs = fee_structures::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee structure not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_structure:read", Some(fs.organization_id), None).await?;

    Ok(Json(to_response(fs)))
}

// PUT /api/v1/fee-structures/{id}
pub async fn update_fee_structure(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateFeeStructureRequest>,
) -> Result<Json<FeeStructureResponse>, (StatusCode, String)> {
    let fs = fee_structures::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee structure not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_structure:update", Some(fs.organization_id), None).await?;

    let mut active_model: ActiveModel = fs.into();

    if let Some(branch_id) = payload.branch_id {
        active_model.branch_id = ActiveValue::Set(Some(branch_id));
    }
    if let Some(class_level_id) = payload.class_level_id {
        active_model.class_level_id = ActiveValue::Set(Some(class_level_id));
    }
    if let Some(master_class_id) = payload.master_class_id {
        active_model.master_class_id = ActiveValue::Set(Some(master_class_id));
    }
    if let Some(student_id) = payload.student_id {
        active_model.student_id = ActiveValue::Set(Some(student_id));
    }
    if let Some(fee_type_id) = payload.fee_type_id {
        active_model.fee_type_id = ActiveValue::Set(fee_type_id);
    }
    if let Some(academic_year_id) = payload.academic_year_id {
        active_model.academic_year_id = ActiveValue::Set(academic_year_id);
    }
    if let Some(amount) = payload.amount {
        active_model.amount = ActiveValue::Set(amount);
    }
    if let Some(due_day) = payload.due_day {
        active_model.due_day = ActiveValue::Set(due_day);
    }
    if let Some(bill_generation_frequency) = payload.bill_generation_frequency {
        active_model.bill_generation_frequency = ActiveValue::Set(bill_generation_frequency);
    }
    if let Some(late_fee_type) = payload.late_fee_type {
        active_model.late_fee_type = ActiveValue::Set(Some(late_fee_type));
    }
    if let Some(late_fee_value) = payload.late_fee_value {
        active_model.late_fee_value = ActiveValue::Set(Some(late_fee_value));
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

// DELETE /api/v1/fee-structures/{id}
pub async fn delete_fee_structure(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let fs = fee_structures::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Fee structure not found".to_string()))?;

    authorize(&*state.db, &claims, "fee_structure:delete", Some(fs.organization_id), None).await?;

    let active_model: ActiveModel = fs.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
