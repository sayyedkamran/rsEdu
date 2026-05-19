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
    entities::discounts::{self, ActiveModel},
    discounts::dto::{CreateDiscountRequest, DiscountResponse, UpdateDiscountRequest},
};

fn to_response(d: discounts::Model) -> DiscountResponse {
    DiscountResponse {
        id: d.id,
        organization_id: d.organization_id,
        name: d.name,
        name_urdu: d.name_urdu,
        discount_type: d.discount_type,
        value: d.value,
        description: d.description,
        requires_approval: d.requires_approval,
        is_active: d.is_active,
    }
}

// POST /api/v1/discounts
pub async fn create_discount(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateDiscountRequest>,
) -> Result<Json<DiscountResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "discount:create", Some(payload.organization_id), None).await?;

    let new_d = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        discount_type: ActiveValue::Set(payload.discount_type),
        value: ActiveValue::Set(payload.value),
        description: ActiveValue::Set(payload.description),
        requires_approval: ActiveValue::Set(payload.requires_approval),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let d = new_d
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(d)))
}

// GET /api/v1/discounts
pub async fn get_discounts(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<DiscountResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "discount:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = discounts::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(discounts::Column::OrganizationId.eq(org_id));
    }

    let discounts = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(discounts.into_iter().map(to_response).collect()))
}

// GET /api/v1/discounts/{id}
pub async fn get_discount(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<DiscountResponse>, (StatusCode, String)> {
    let d = discounts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Discount not found".to_string()))?;

    authorize(&*state.db, &claims, "discount:read", Some(d.organization_id), None).await?;

    Ok(Json(to_response(d)))
}

// PUT /api/v1/discounts/{id}
pub async fn update_discount(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateDiscountRequest>,
) -> Result<Json<DiscountResponse>, (StatusCode, String)> {
    let d = discounts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Discount not found".to_string()))?;

    authorize(&*state.db, &claims, "discount:update", Some(d.organization_id), None).await?;

    let mut active_model: ActiveModel = d.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(discount_type) = payload.discount_type {
        active_model.discount_type = ActiveValue::Set(discount_type);
    }
    if let Some(value) = payload.value {
        active_model.value = ActiveValue::Set(value);
    }
    if let Some(description) = payload.description {
        active_model.description = ActiveValue::Set(Some(description));
    }
    if let Some(requires_approval) = payload.requires_approval {
        active_model.requires_approval = ActiveValue::Set(requires_approval);
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

// DELETE /api/v1/discounts/{id}
pub async fn delete_discount(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let d = discounts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Discount not found".to_string()))?;

    authorize(&*state.db, &claims, "discount:delete", Some(d.organization_id), None).await?;

    let active_model: ActiveModel = d.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
