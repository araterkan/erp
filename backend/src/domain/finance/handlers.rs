use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension,
    Json,
};
use uuid::Uuid;

use crate::error::AppResult;
use crate::AppState;
use crate::domain::auth::models::Claims;
use super::models::{CreateAccountRequest, CreateJournalEntryRequest};
use super::service::FinanceService;

pub async fn list_accounts(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    let accounts = FinanceService::list_accounts(&state.pool).await?;
    Ok(Json(accounts))
}

pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let account = FinanceService::get_account(&state.pool, id).await?;
    Ok(Json(account))
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(req): Json<CreateAccountRequest>,
) -> AppResult<impl IntoResponse> {
    let account = FinanceService::create_account(&state.pool, &req).await?;
    Ok((StatusCode::CREATED, Json(account)))
}

pub async fn list_journal_entries(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    let entries = FinanceService::list_journal_entries(&state.pool).await?;
    Ok(Json(entries))
}

pub async fn create_journal_entry(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateJournalEntryRequest>,
) -> AppResult<impl IntoResponse> {
    let entry = FinanceService::create_journal_entry(&state.pool, &req, claims.user_id).await?;
    Ok((StatusCode::CREATED, Json(entry)))
}

pub async fn get_trial_balance(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    let tb = FinanceService::get_trial_balance(&state.pool).await?;
    Ok(Json(tb))
}
