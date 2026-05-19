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
    entities::class_levels::{self, ActiveModel},
    class_levels::dto::{CreateClassLevelRequest, ClassLevelResponse, UpdateClassLevelRequest},
};

fn to_response(cl: class_levels::Model) -> ClassLevelResponse {
    ClassLevelResponse {
        id: cl.id,
        organization_id: cl.organization_id,
        name: cl.name,
        name_urdu: cl.name_urdu,
        order: cl.order,
        description: cl.description,
        is_active: cl.is_active,
    }
}

// POST /api/v1/class-levels
pub async fn create_class_level(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateClassLevelRequest>,
) -> Result<Json<ClassLevelResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "class_level:create", Some(payload.organization_id), None).await?;

    let new_cl = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        order: ActiveValue::Set(payload.order),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let cl = new_cl
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(cl)))
}

// GET /api/v1/class-levels
pub async fn get_class_levels(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ClassLevelResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "class_level:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = class_levels::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(class_levels::Column::OrganizationId.eq(org_id));
    }

    let class_levels = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(class_levels.into_iter().map(to_response).collect()))
}

// GET /api/v1/class-levels/{id}
pub async fn get_class_level(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ClassLevelResponse>, (StatusCode, String)> {
    let cl = class_levels::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class level not found".to_string()))?;

    authorize(&*state.db, &claims, "class_level:read", Some(cl.organization_id), None).await?;

    Ok(Json(to_response(cl)))
}

// PUT /api/v1/class-levels/{id}
pub async fn update_class_level(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateClassLevelRequest>,
) -> Result<Json<ClassLevelResponse>, (StatusCode, String)> {
    let cl = class_levels::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class level not found".to_string()))?;

    authorize(&*state.db, &claims, "class_level:update", Some(cl.organization_id), None).await?;

    let mut active_model: ActiveModel = cl.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(order) = payload.order {
        active_model.order = ActiveValue::Set(order);
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

// DELETE /api/v1/class-levels/{id}
pub async fn delete_class_level(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let cl = class_levels::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Class level not found".to_string()))?;

    authorize(&*state.db, &claims, "class_level:delete", Some(cl.organization_id), None).await?;

    let active_model: ActiveModel = cl.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}