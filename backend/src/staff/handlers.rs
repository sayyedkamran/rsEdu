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
    entities::staff::{self, ActiveModel},
    staff::dto::{CreateStaffRequest, StaffResponse, UpdateStaffRequest},
};

fn to_response(s: staff::Model) -> StaffResponse {
    StaffResponse {
        id: s.id,
        user_id: s.user_id,
        organization_id: s.organization_id,
        branch_id: s.branch_id,
        staff_type_id: s.staff_type_id,
        first_name: s.first_name,
        last_name: s.last_name,
        father_name: s.father_name,
        date_of_birth: s.date_of_birth.to_string(),
        gender: s.gender,
        cnic: s.cnic,
        joining_date: s.joining_date.to_string(),
        is_active: s.is_active,
    }
}

// POST /api/v1/staff
pub async fn create_staff(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStaffRequest>,
) -> Result<Json<StaffResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff:create", Some(payload.organization_id), None).await?;

    let date_of_birth = NaiveDate::parse_from_str(&payload.date_of_birth, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid date_of_birth: {}", e)))?;

    let joining_date = NaiveDate::parse_from_str(&payload.joining_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid joining_date: {}", e)))?;

    let new_staff = ActiveModel {
        user_id: ActiveValue::Set(payload.user_id),
        organization_id: ActiveValue::Set(payload.organization_id),
        branch_id: ActiveValue::Set(payload.branch_id),
        staff_type_id: ActiveValue::Set(payload.staff_type_id),
        first_name: ActiveValue::Set(payload.first_name),
        last_name: ActiveValue::Set(payload.last_name),
        father_name: ActiveValue::Set(payload.father_name),
        date_of_birth: ActiveValue::Set(date_of_birth),
        gender: ActiveValue::Set(payload.gender),
        cnic: ActiveValue::Set(payload.cnic),
        joining_date: ActiveValue::Set(joining_date),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let s = new_staff
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(s)))
}

// GET /api/v1/staff
pub async fn get_all_staff(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<StaffResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "staff:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = staff::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(staff::Column::OrganizationId.eq(org_id));
    }

    let staff = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(staff.into_iter().map(to_response).collect()))
}

// GET /api/v1/staff/{id}
pub async fn get_staff_member(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<StaffResponse>, (StatusCode, String)> {
    let s = staff::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff member not found".to_string()))?;

    authorize(&*state.db, &claims, "staff:read", Some(s.organization_id), None).await?;

    Ok(Json(to_response(s)))
}

// PUT /api/v1/staff/{id}
pub async fn update_staff(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateStaffRequest>,
) -> Result<Json<StaffResponse>, (StatusCode, String)> {
    let s = staff::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff member not found".to_string()))?;

    authorize(&*state.db, &claims, "staff:update", Some(s.organization_id), None).await?;

    let mut active_model: ActiveModel = s.into();

    if let Some(branch_id) = payload.branch_id {
        active_model.branch_id = ActiveValue::Set(Some(branch_id));
    }
    if let Some(staff_type_id) = payload.staff_type_id {
        active_model.staff_type_id = ActiveValue::Set(staff_type_id);
    }
    if let Some(first_name) = payload.first_name {
        active_model.first_name = ActiveValue::Set(first_name);
    }
    if let Some(last_name) = payload.last_name {
        active_model.last_name = ActiveValue::Set(last_name);
    }
    if let Some(father_name) = payload.father_name {
        active_model.father_name = ActiveValue::Set(Some(father_name));
    }
    if let Some(date_of_birth) = payload.date_of_birth {
        let date = NaiveDate::parse_from_str(&date_of_birth, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid date_of_birth: {}", e)))?;
        active_model.date_of_birth = ActiveValue::Set(date);
    }
    if let Some(gender) = payload.gender {
        active_model.gender = ActiveValue::Set(gender);
    }
    if let Some(cnic) = payload.cnic {
        active_model.cnic = ActiveValue::Set(Some(cnic));
    }
    if let Some(joining_date) = payload.joining_date {
        let date = NaiveDate::parse_from_str(&joining_date, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid joining_date: {}", e)))?;
        active_model.joining_date = ActiveValue::Set(date);
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

// DELETE /api/v1/staff/{id}
pub async fn delete_staff(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let s = staff::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Staff member not found".to_string()))?;

    authorize(&*state.db, &claims, "staff:delete", Some(s.organization_id), None).await?;

    let active_model: ActiveModel = s.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
