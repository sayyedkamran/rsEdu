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
    entities::expense_categories::{self, ActiveModel},
    expense_categories::dto::{CreateExpenseCategoryRequest, ExpenseCategoryResponse, UpdateExpenseCategoryRequest},
};

fn to_response(ec: expense_categories::Model) -> ExpenseCategoryResponse {
    ExpenseCategoryResponse {
        id: ec.id,
        organization_id: ec.organization_id,
        name: ec.name,
        name_urdu: ec.name_urdu,
        description: ec.description,
        is_active: ec.is_active,
    }
}

// POST /api/v1/expense-categories
pub async fn create_expense_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateExpenseCategoryRequest>,
) -> Result<Json<ExpenseCategoryResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "expense_category:create", Some(payload.organization_id), None).await?;

    let new_ec = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let ec = new_ec
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(ec)))
}

// GET /api/v1/expense-categories
pub async fn get_expense_categories(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ExpenseCategoryResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "expense_category:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = expense_categories::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(expense_categories::Column::OrganizationId.eq(org_id));
    }

    let records = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/expense-categories/{id}
pub async fn get_expense_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ExpenseCategoryResponse>, (StatusCode, String)> {
    let ec = expense_categories::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Expense category not found".to_string()))?;

    authorize(&*state.db, &claims, "expense_category:read", Some(ec.organization_id), None).await?;

    Ok(Json(to_response(ec)))
}

// PUT /api/v1/expense-categories/{id}
pub async fn update_expense_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateExpenseCategoryRequest>,
) -> Result<Json<ExpenseCategoryResponse>, (StatusCode, String)> {
    let ec = expense_categories::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Expense category not found".to_string()))?;

    authorize(&*state.db, &claims, "expense_category:update", Some(ec.organization_id), None).await?;

    let mut active_model: ActiveModel = ec.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(description) = payload.description {
        active_model.description = ActiveValue::Set(Some(description));
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

// DELETE /api/v1/expense-categories/{id}
pub async fn delete_expense_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let ec = expense_categories::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Expense category not found".to_string()))?;

    authorize(&*state.db, &claims, "expense_category:delete", Some(ec.organization_id), None).await?;

    let active_model: ActiveModel = ec.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
