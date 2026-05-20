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
    entities::contacts::{self, ActiveModel},
    contacts::dto::{CreateContactRequest, ContactResponse, UpdateContactRequest},
};

fn to_response(c: contacts::Model) -> ContactResponse {
    ContactResponse {
        id: c.id,
        entity_type: c.entity_type,
        entity_id: c.entity_id,
        contact_type: c.contact_type,
        value: c.value,
        has_whatsapp: c.has_whatsapp,
        is_primary: c.is_primary,
        is_active: c.is_active,
    }
}

// POST /api/v1/contacts
pub async fn create_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateContactRequest>,
) -> Result<Json<ContactResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "contact:create", None, None).await?;

    let new_contact = ActiveModel {
        entity_type: ActiveValue::Set(payload.entity_type),
        entity_id: ActiveValue::Set(payload.entity_id),
        contact_type: ActiveValue::Set(payload.contact_type),
        value: ActiveValue::Set(payload.value),
        has_whatsapp: ActiveValue::Set(payload.has_whatsapp),
        is_primary: ActiveValue::Set(payload.is_primary),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let contact = new_contact
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(contact)))
}

// GET /api/v1/contacts
pub async fn get_contacts(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ContactResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "contact:read", None, None).await?;

    let records = contacts::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(records.into_iter().map(to_response).collect()))
}

// GET /api/v1/contacts/{id}
pub async fn get_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ContactResponse>, (StatusCode, String)> {
    let contact = contacts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Contact not found".to_string()))?;

    authorize(&*state.db, &claims, "contact:read", None, None).await?;

    Ok(Json(to_response(contact)))
}

// PUT /api/v1/contacts/{id}
pub async fn update_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateContactRequest>,
) -> Result<Json<ContactResponse>, (StatusCode, String)> {
    let contact = contacts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Contact not found".to_string()))?;

    authorize(&*state.db, &claims, "contact:update", None, None).await?;

    let mut active_model: ActiveModel = contact.into();

    if let Some(contact_type) = payload.contact_type {
        active_model.contact_type = ActiveValue::Set(contact_type);
    }
    if let Some(value) = payload.value {
        active_model.value = ActiveValue::Set(value);
    }
    if let Some(has_whatsapp) = payload.has_whatsapp {
        active_model.has_whatsapp = ActiveValue::Set(has_whatsapp);
    }
    if let Some(is_primary) = payload.is_primary {
        active_model.is_primary = ActiveValue::Set(is_primary);
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

// DELETE /api/v1/contacts/{id}
pub async fn delete_contact(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let contact = contacts::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Contact not found".to_string()))?;

    authorize(&*state.db, &claims, "contact:delete", None, None).await?;

    let active_model: ActiveModel = contact.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
