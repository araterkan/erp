use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;
use crate::error::AppResult;
use crate::AppState;
use super::models::*;
use super::service::CrmService;

pub async fn list_customers(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(CrmService::list_customers(&state.pool).await?))
}

pub async fn get_customer(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<impl IntoResponse> {
    Ok(Json(CrmService::get_customer(&state.pool, id).await?))
}

pub async fn create_customer(
    State(state): State<AppState>,
    Json(req): Json<CreateCustomerRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(CrmService::create_customer(&state.pool, &req).await?)))
}

pub async fn list_contacts(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(CrmService::list_contacts(&state.pool).await?))
}

pub async fn create_contact(
    State(state): State<AppState>,
    Json(req): Json<CreateContactRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(CrmService::create_contact(&state.pool, &req).await?)))
}
