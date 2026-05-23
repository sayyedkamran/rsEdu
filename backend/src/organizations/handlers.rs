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
    entities::organizations::{self, ActiveModel},
    organizations::dto::{CreateOrganizationRequest, OrganizationResponse, UpdateOrganizationRequest},
};

fn to_response(org: organizations::Model) -> OrganizationResponse {
    OrganizationResponse {
        id: org.id,
        name: org.name,
        name_urdu: org.name_urdu,
        logo_path: org.logo_path,
        website: org.website,
        email: org.email,
        phone: org.phone,
        city_id: org.city_id,
        address: org.address,
        is_active: org.is_active,
    }
}

// POST /api/v1/organizations
pub async fn create_organization(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateOrganizationRequest>,
) -> Result<Json<OrganizationResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "organization:create", None, None).await?;

    let new_org = ActiveModel {
        name: ActiveValue::Set(payload.name),
        name_urdu: ActiveValue::Set(payload.name_urdu),
        logo_path: ActiveValue::Set(payload.logo_path),
        website: ActiveValue::Set(payload.website),
        email: ActiveValue::Set(payload.email),
        phone: ActiveValue::Set(payload.phone),
        city_id: ActiveValue::Set(payload.city_id),
        address: ActiveValue::Set(payload.address),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let org = new_org
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(org)))
}

// GET /api/v1/organizations
pub async fn get_organizations(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<OrganizationResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "organization:read", None, None).await?;

    let orgs = organizations::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(orgs.into_iter().map(to_response).collect()))
}

// GET /api/v1/organizations/{id}
pub async fn get_organization(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<OrganizationResponse>, (StatusCode, String)> {
    let org = organizations::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Organization not found".to_string()))?;

    authorize(&*state.db, &claims, "organization:read", Some(org.id), None).await?;

    Ok(Json(to_response(org)))
}

// PUT /api/v1/organizations/{id}
pub async fn update_organization(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateOrganizationRequest>,
) -> Result<Json<OrganizationResponse>, (StatusCode, String)> {
    let org = organizations::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Organization not found".to_string()))?;

    authorize(&*state.db, &claims, "organization:update", Some(org.id), None).await?;

    let mut active_model: ActiveModel = org.into();

    if let Some(name) = payload.name {
        active_model.name = ActiveValue::Set(name);
    }
    if let Some(name_urdu) = payload.name_urdu {
        active_model.name_urdu = ActiveValue::Set(Some(name_urdu));
    }
    if let Some(logo_path) = payload.logo_path {
    active_model.logo_path = ActiveValue::Set(Some(logo_path));
    }
    if let Some(website) = payload.website {
        active_model.website = ActiveValue::Set(Some(website));
    }
    if let Some(email) = payload.email {
        active_model.email = ActiveValue::Set(Some(email));
    }
    if let Some(phone) = payload.phone {
        active_model.phone = ActiveValue::Set(Some(phone));
    }
    if let Some(city_id) = payload.city_id {
        active_model.city_id = ActiveValue::Set(Some(city_id));
    }
    if let Some(address) = payload.address {
        active_model.address = ActiveValue::Set(Some(address));
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

// DELETE /api/v1/organizations/{id}
pub async fn delete_organization(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let org = organizations::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Organization not found".to_string()))?;

    authorize(&*state.db, &claims, "organization:delete", Some(org.id), None).await?;

    let active_model: ActiveModel = org.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}