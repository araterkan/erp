use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use uuid::Uuid;
use crate::error::AppResult;
use crate::AppState;
use crate::domain::auth::models::Claims;
use super::models::*;
use super::service::BankService;

pub async fn list_accounts(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(BankService::list_accounts(&state.pool).await?))
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(req): Json<CreateBankAccountRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(BankService::create_account(&state.pool, &req).await?)))
}

pub async fn list_transactions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    Ok(Json(BankService::list_transactions(&state.pool, id).await?))
}

pub async fn create_transaction(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateBankTransactionRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(BankService::create_transaction(&state.pool, &req, claims.user_id).await?)))
}
