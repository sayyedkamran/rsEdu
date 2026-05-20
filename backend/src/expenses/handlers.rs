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
    entities::expenses::{self, ActiveModel},
    expenses::dto::{CreateExpenseRequest, ExpenseResponse, UpdateExpenseRequest},
};

fn to_response(e: expenses::Model) -> ExpenseResponse {
    ExpenseResponse {
        id: e.id,
        branch_id: e.branch_id,
        expense_category_id: e.expense_category_id,
        amount: e.amount,
        description: e.description,
        expense_date: e.expense_date.to_string(),
        receipt_number: e.receipt_number,
        payment_method_id: e.payment_method_id,
        reference_number: e.reference_number,
        approved_by: e.approved_by,
        entered_by: e.entered_by,
        remarks: e.remarks,
    }
}

// POST /api/v1/expenses
pub async fn create_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateExpenseRequest>,
) -> Result<Json<ExpenseResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "expense:create", None, None).await?;

    let expense_date = NaiveDate::parse_from_str(&payload.expense_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid expense_date: {}", e)))?;

    let new_expense = ActiveModel {
        branch_id: ActiveValue::Set(payload.branch_id),
        expense_category_id: ActiveValue::Set(payload.expense_category_id),
        amount: ActiveValue::Set(payload.amount),
        description: ActiveValue::Set(payload.description),
        expense_date: ActiveValue::Set(expense_date),
        receipt_number: ActiveValue::Set(payload.receipt_number),
        payment_method_id: ActiveValue::Set(payload.payment_method_id),
        reference_number: ActiveValue::Set(payload.reference_number),
        approved_by: ActiveValue::Set(payload.approved_by),
        entered_by: ActiveValue::Set(payload.entered_by),
        remarks: ActiveValue::Set(payload.remarks),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let expense = new_expense
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(expense)))
}

// GET /api/v1/expenses
pub async fn get_expenses(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ExpenseResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "expense:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = expenses::Entity::find();

    if let Some(branch_id) = scope.branch_id {
        query = query.filter(expenses::Column::BranchId.eq(branch_id));
    }

    let records = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/expenses/{id}
pub async fn get_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ExpenseResponse>, (StatusCode, String)> {
    let expense = expenses::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Expense not found".to_string()))?;

    authorize(&*state.db, &claims, "expense:read", None, None).await?;

    Ok(Json(to_response(expense)))
}

// PUT /api/v1/expenses/{id}
pub async fn update_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateExpenseRequest>,
) -> Result<Json<ExpenseResponse>, (StatusCode, String)> {
    let expense = expenses::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Expense not found".to_string()))?;

    authorize(&*state.db, &claims, "expense:update", None, None).await?;

    let mut active_model: ActiveModel = expense.into();

    if let Some(amount) = payload.amount {
        active_model.amount = ActiveValue::Set(amount);
    }
    if let Some(description) = payload.description {
        active_model.description = ActiveValue::Set(description);
    }
    if let Some(expense_date_str) = payload.expense_date {
        let date = NaiveDate::parse_from_str(&expense_date_str, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid expense_date: {}", e)))?;
        active_model.expense_date = ActiveValue::Set(date);
    }
    if let Some(receipt_number) = payload.receipt_number {
        active_model.receipt_number = ActiveValue::Set(Some(receipt_number));
    }
    if let Some(reference_number) = payload.reference_number {
        active_model.reference_number = ActiveValue::Set(Some(reference_number));
    }
    if let Some(approved_by) = payload.approved_by {
        active_model.approved_by = ActiveValue::Set(Some(approved_by));
    }
    if let Some(remarks) = payload.remarks {
        active_model.remarks = ActiveValue::Set(Some(remarks));
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/expenses/{id}
pub async fn delete_expense(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let expense = expenses::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Expense not found".to_string()))?;

    authorize(&*state.db, &claims, "expense:delete", None, None).await?;

    let active_model: ActiveModel = expense.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
