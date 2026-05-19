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
    entities::academic_years::{self, ActiveModel},
    academic_years::dto::{CreateAcademicYearRequest, AcademicYearResponse, UpdateAcademicYearRequest},
};

fn to_response(ay: academic_years::Model) -> AcademicYearResponse {
    AcademicYearResponse {
        id: ay.id,
        organization_id: ay.organization_id,
        stream_id: ay.stream_id,
        title: ay.title,
        start_date: ay.start_date.to_string(),
        end_date: ay.end_date.to_string(),
        description: ay.description,
        is_active: ay.is_active,
    }
}

// POST /api/v1/academic-years
pub async fn create_academic_year(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateAcademicYearRequest>,
) -> Result<Json<AcademicYearResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "academic_year:create", Some(payload.organization_id), None).await?;

    let start_date = NaiveDate::parse_from_str(&payload.start_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid start_date: {}", e)))?;

    let end_date = NaiveDate::parse_from_str(&payload.end_date, "%Y-%m-%d")
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid end_date: {}", e)))?;

    let new_ay = ActiveModel {
        organization_id: ActiveValue::Set(payload.organization_id),
        stream_id: ActiveValue::Set(payload.stream_id),
        title: ActiveValue::Set(payload.title),
        start_date: ActiveValue::Set(start_date),
        end_date: ActiveValue::Set(end_date),
        description: ActiveValue::Set(payload.description),
        is_active: ActiveValue::Set(true),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let ay = new_ay
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(ay)))
}

// GET /api/v1/academic-years
pub async fn get_academic_years(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<AcademicYearResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "academic_year:read", None, None).await?;

    let scope = get_data_scope(&claims);
    let mut query = academic_years::Entity::find();

    if let Some(org_id) = scope.organization_id {
        query = query.filter(academic_years::Column::OrganizationId.eq(org_id));
    }

    let academic_years = query
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(academic_years.into_iter().map(to_response).collect()))
}

// GET /api/v1/academic-years/{id}
pub async fn get_academic_year(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<AcademicYearResponse>, (StatusCode, String)> {
    let ay = academic_years::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Academic year not found".to_string()))?;

    authorize(&*state.db, &claims, "academic_year:read", Some(ay.organization_id), None).await?;

    Ok(Json(to_response(ay)))
}

// PUT /api/v1/academic-years/{id}
pub async fn update_academic_year(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateAcademicYearRequest>,
) -> Result<Json<AcademicYearResponse>, (StatusCode, String)> {
    let ay = academic_years::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Academic year not found".to_string()))?;

    authorize(&*state.db, &claims, "academic_year:update", Some(ay.organization_id), None).await?;

    let mut active_model: ActiveModel = ay.into();

    if let Some(title) = payload.title {
        active_model.title = ActiveValue::Set(title);
    }
    if let Some(start_date) = payload.start_date {
        let date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid start_date: {}", e)))?;
        active_model.start_date = ActiveValue::Set(date);
    }
    if let Some(end_date) = payload.end_date {
        let date = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid end_date: {}", e)))?;
        active_model.end_date = ActiveValue::Set(date);
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

// DELETE /api/v1/academic-years/{id}
pub async fn delete_academic_year(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let ay = academic_years::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Academic year not found".to_string()))?;

    authorize(&*state.db, &claims, "academic_year:delete", Some(ay.organization_id), None).await?;

    let active_model: ActiveModel = ay.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}