use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use uuid::Uuid;
use crate::error::AppResult;
use crate::AppState;
use crate::domain::auth::models::Claims;
use super::models::*;
use super::service::HrService;

pub async fn list_departments(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(HrService::list_departments(&state.pool).await?))
}

pub async fn create_department(
    State(state): State<AppState>,
    Json(req): Json<CreateDepartmentRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(HrService::create_department(&state.pool, &req).await?)))
}

pub async fn list_employees(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(HrService::list_employees(&state.pool).await?))
}

pub async fn get_employee(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<impl IntoResponse> {
    Ok(Json(HrService::get_employee(&state.pool, id).await?))
}

pub async fn create_employee(
    State(state): State<AppState>,
    Json(req): Json<CreateEmployeeRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(HrService::create_employee(&state.pool, &req).await?)))
}

pub async fn list_leave_requests(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(HrService::list_leave_requests(&state.pool).await?))
}

pub async fn create_leave_request(
    State(state): State<AppState>,
    Json(req): Json<CreateLeaveRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(HrService::create_leave_request(&state.pool, &req).await?)))
}

pub async fn list_payroll_runs(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(HrService::list_payroll_runs(&state.pool).await?))
}

pub async fn create_payroll_run(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreatePayrollRunRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(HrService::create_payroll_run(&state.pool, &req, claims.user_id).await?)))
}
