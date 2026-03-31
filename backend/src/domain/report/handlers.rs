use axum::{extract::State, response::IntoResponse, Json};
use crate::error::AppResult;
use crate::AppState;
use super::service::ReportService;

pub async fn dashboard(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(ReportService::get_dashboard_stats(&state.pool).await?))
}

pub async fn sales_report(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(ReportService::get_sales_report(&state.pool).await?))
}

pub async fn inventory_report(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    Ok(Json(ReportService::get_inventory_report(&state.pool).await?))
}
