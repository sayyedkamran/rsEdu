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
    entities::results::{self, ActiveModel},
    results::dto::{CreateResultRequest, ResultResponse, UpdateResultRequest},
};

fn to_response(r: results::Model) -> ResultResponse {
    ResultResponse {
        id: r.id,
        exam_subject_id: r.exam_subject_id,
        student_id: r.student_id,
        marks_obtained: r.marks_obtained,
        is_absent: r.is_absent,
        is_exempt: r.is_exempt,
        grade: r.grade,
        remarks: r.remarks,
        entered_by: r.entered_by,
    }
}

// POST /api/v1/results
pub async fn create_result(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateResultRequest>,
) -> Result<Json<ResultResponse>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "result:create", None, None).await?;

    let new_result = ActiveModel {
        exam_subject_id: ActiveValue::Set(payload.exam_subject_id),
        student_id: ActiveValue::Set(payload.student_id),
        marks_obtained: ActiveValue::Set(payload.marks_obtained),
        is_absent: ActiveValue::Set(payload.is_absent),
        is_exempt: ActiveValue::Set(payload.is_exempt),
        grade: ActiveValue::Set(payload.grade),
        remarks: ActiveValue::Set(payload.remarks),
        entered_by: ActiveValue::Set(payload.entered_by),
        created_at: ActiveValue::Set(Utc::now().into()),
        updated_at: ActiveValue::Set(Utc::now().into()),
        ..Default::default()
    };

    let r = new_result
        .insert(&*state.db)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(to_response(r)))
}

// GET /api/v1/results
pub async fn get_results(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ResultResponse>>, (StatusCode, String)> {
    authorize(&*state.db, &claims, "result:read", None, None).await?;

    let results = results::Entity::find()
        .all(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(results.into_iter().map(to_response).collect()))
}

// GET /api/v1/results/{id}
pub async fn get_result(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<Json<ResultResponse>, (StatusCode, String)> {
    let r = results::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Result not found".to_string()))?;

    authorize(&*state.db, &claims, "result:read", None, None).await?;

    Ok(Json(to_response(r)))
}

// PUT /api/v1/results/{id}
pub async fn update_result(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateResultRequest>,
) -> Result<Json<ResultResponse>, (StatusCode, String)> {
    let r = results::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Result not found".to_string()))?;

    authorize(&*state.db, &claims, "result:update", None, None).await?;

    let mut active_model: ActiveModel = r.into();

    if let Some(marks_obtained) = payload.marks_obtained {
        active_model.marks_obtained = ActiveValue::Set(Some(marks_obtained));
    }
    if let Some(is_absent) = payload.is_absent {
        active_model.is_absent = ActiveValue::Set(is_absent);
    }
    if let Some(is_exempt) = payload.is_exempt {
        active_model.is_exempt = ActiveValue::Set(is_exempt);
    }
    if let Some(grade) = payload.grade {
        active_model.grade = ActiveValue::Set(Some(grade));
    }
    if let Some(remarks) = payload.remarks {
        active_model.remarks = ActiveValue::Set(Some(remarks));
    }
    if let Some(entered_by) = payload.entered_by {
        active_model.entered_by = ActiveValue::Set(entered_by);
    }

    active_model.updated_at = ActiveValue::Set(Utc::now().into());

    let updated = active_model
        .update(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(to_response(updated)))
}

// DELETE /api/v1/results/{id}
pub async fn delete_result(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let r = results::Entity::find_by_id(id)
        .one(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Result not found".to_string()))?;

    authorize(&*state.db, &claims, "result:delete", None, None).await?;

    let active_model: ActiveModel = r.into();

    active_model
        .delete(&*state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
