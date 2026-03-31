use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Extension, Json};
use uuid::Uuid;
use crate::error::AppResult;
use crate::AppState;
use crate::domain::auth::models::Claims;
use super::models::{CreateProductRequest, CreateStockMovementRequest, CreateWarehouseRequest};
use super::service::StockService;

pub async fn list_warehouses(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(StockService::list_warehouses(&state.pool).await?))
}

pub async fn create_warehouse(
    State(state): State<AppState>,
    Json(req): Json<CreateWarehouseRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(StockService::create_warehouse(&state.pool, &req).await?)))
}

pub async fn list_products(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(StockService::list_products(&state.pool).await?))
}

pub async fn get_product(State(state): State<AppState>, Path(id): Path<Uuid>) -> AppResult<impl IntoResponse> {
    Ok(Json(StockService::get_product(&state.pool, id).await?))
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(req): Json<CreateProductRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(StockService::create_product(&state.pool, &req).await?)))
}

pub async fn create_stock_movement(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateStockMovementRequest>,
) -> AppResult<impl IntoResponse> {
    Ok((StatusCode::CREATED, Json(StockService::create_stock_movement(&state.pool, &req, claims.user_id).await?)))
}

pub async fn get_stock_levels(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(StockService::get_stock_levels(&state.pool).await?))
}
