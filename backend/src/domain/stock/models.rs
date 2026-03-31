use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Warehouse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWarehouseRequest {
    pub code: String,
    pub name: String,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub unit: String,
    pub purchase_price: Option<f64>,
    pub sale_price: Option<f64>,
    pub tax_rate: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub unit: Option<String>,
    pub purchase_price: Option<f64>,
    pub sale_price: Option<f64>,
    pub tax_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockMovement {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub movement_type: String,
    pub quantity: f64,
    pub unit_cost: Option<f64>,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateStockMovementRequest {
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub movement_type: String,
    pub quantity: f64,
    pub unit_cost: Option<f64>,
    pub reference: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockLevel {
    pub product_id: Uuid,
    pub product_name: String,
    pub sku: String,
    pub warehouse_id: Uuid,
    pub warehouse_name: String,
    pub quantity: f64,
    pub updated_at: DateTime<Utc>,
}
