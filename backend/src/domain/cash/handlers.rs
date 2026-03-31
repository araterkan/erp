use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use uuid::Uuid;
use crate::error::AppResult;
use crate::AppState;
use crate::domain::auth::models::Claims;
use super::models::*;
use super::service::CashService;

pub async fn list_registers(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(CashService::list_registers(&state.pool).await?))
}

pub async fn create_register(
    State(state): State<AppState>,
    Json(req): Json<CreateCashRegisterRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(CashService::create_register(&state.pool, &req).await?)))
}

pub async fn list_transactions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    Ok(Json(CashService::list_transactions(&state.pool, id).await?))
}

pub async fn create_transaction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateCashTransactionRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(CashService::create_transaction(&state.pool, &req, claims.user_id).await?)))
}
