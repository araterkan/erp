use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use uuid::Uuid;
use crate::error::AppResult;
use crate::AppState;
use crate::domain::auth::models::Claims;
use super::models::CreateInvoiceRequest;
use super::service::InvoiceService;

pub async fn list_invoices(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(InvoiceService::list_invoices(&state.pool).await?))
}

pub async fn get_invoice(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<impl IntoResponse> {
    Ok(Json(InvoiceService::get_invoice(&state.pool, id).await?))
}

pub async fn create_invoice(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateInvoiceRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(InvoiceService::create_invoice(&state.pool, &req, claims.user_id).await?)))
}
