use uuid::Uuid;
use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use super::models::{
    CreateProductRequest, CreateStockMovementRequest, CreateWarehouseRequest,
    Product, StockLevel, StockMovement, Warehouse,
};

pub struct StockService;

impl StockService {
    pub async fn list_warehouses(pool: &DbPool) -> AppResult<Vec<Warehouse>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, code, name, address, is_active, created_at, updated_at FROM warehouses ORDER BY name",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| Warehouse {
            id: r.get("id"), code: r.get("code"), name: r.get("name"),
            address: r.get("address"), is_active: r.get("is_active"),
            created_at: r.get("created_at"), updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn create_warehouse(pool: &DbPool, req: &CreateWarehouseRequest) -> AppResult<Warehouse> {
        let client = pool.get().await?;
        let row = client.query_one(
            "INSERT INTO warehouses (code, name, address) VALUES ($1, $2, $3)
             RETURNING id, code, name, address, is_active, created_at, updated_at",
            &[&req.code, &req.name, &req.address],
        ).await?;
        Ok(Warehouse {
            id: row.get("id"), code: row.get("code"), name: row.get("name"),
            address: row.get("address"), is_active: row.get("is_active"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
        })
    }

    pub async fn list_products(pool: &DbPool) -> AppResult<Vec<Product>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, sku, name, description, category_id, unit, purchase_price, sale_price, tax_rate, is_active, created_at, updated_at
             FROM products ORDER BY name",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| Product {
            id: r.get("id"), sku: r.get("sku"), name: r.get("name"),
            description: r.get("description"), category_id: r.get("category_id"),
            unit: r.get("unit"),
            purchase_price: r.get::<_, Option<f64>>("purchase_price"),
            sale_price: r.get::<_, Option<f64>>("sale_price"),
            tax_rate: r.get::<_, f64>("tax_rate"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"), updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn get_product(pool: &DbPool, id: Uuid) -> AppResult<Product> {
        let client = pool.get().await?;
        let row = client.query_opt(
            "SELECT id, sku, name, description, category_id, unit, purchase_price, sale_price, tax_rate, is_active, created_at, updated_at
             FROM products WHERE id = $1",
            &[&id],
        ).await?.ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;
        Ok(Product {
            id: row.get("id"), sku: row.get("sku"), name: row.get("name"),
            description: row.get("description"), category_id: row.get("category_id"),
            unit: row.get("unit"),
            purchase_price: row.get::<_, Option<f64>>("purchase_price"),
            sale_price: row.get::<_, Option<f64>>("sale_price"),
            tax_rate: row.get::<_, f64>("tax_rate"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
        })
    }

    pub async fn create_product(pool: &DbPool, req: &CreateProductRequest) -> AppResult<Product> {
        let client = pool.get().await?;
        let unit = req.unit.as_deref().unwrap_or("pcs");
        let tax_rate = req.tax_rate.unwrap_or(0.0);
        let row = client.query_one(
            "INSERT INTO products (sku, name, description, category_id, unit, purchase_price, sale_price, tax_rate)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING id, sku, name, description, category_id, unit, purchase_price, sale_price, tax_rate, is_active, created_at, updated_at",
            &[&req.sku, &req.name, &req.description, &req.category_id, &unit, &req.purchase_price, &req.sale_price, &tax_rate],
        ).await?;
        Ok(Product {
            id: row.get("id"), sku: row.get("sku"), name: row.get("name"),
            description: row.get("description"), category_id: row.get("category_id"),
            unit: row.get("unit"),
            purchase_price: row.get::<_, Option<f64>>("purchase_price"),
            sale_price: row.get::<_, Option<f64>>("sale_price"),
            tax_rate: row.get::<_, f64>("tax_rate"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
        })
    }

    pub async fn create_stock_movement(
        pool: &DbPool,
        req: &CreateStockMovementRequest,
        user_id: Uuid,
    ) -> AppResult<StockMovement> {
        let mut client = pool.get().await?;
        let tx = client.transaction().await?;

        let row = tx.query_one(
            "INSERT INTO stock_movements (product_id, warehouse_id, movement_type, quantity, unit_cost, reference, notes, created_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING id, product_id, warehouse_id, movement_type, quantity, unit_cost, reference, notes, created_by, created_at",
            &[&req.product_id, &req.warehouse_id, &req.movement_type, &req.quantity,
              &req.unit_cost, &req.reference, &req.notes, &user_id],
        ).await?;

        let qty_delta = match req.movement_type.as_str() {
            "in" | "receipt" | "adjustment_in" => req.quantity,
            "out" | "shipment" | "adjustment_out" => -req.quantity,
            _ => req.quantity,
        };

        tx.execute(
            "INSERT INTO stock_levels (product_id, warehouse_id, quantity)
             VALUES ($1, $2, $3)
             ON CONFLICT (product_id, warehouse_id)
             DO UPDATE SET quantity = stock_levels.quantity + $3, updated_at = NOW()",
            &[&req.product_id, &req.warehouse_id, &qty_delta],
        ).await?;

        tx.commit().await?;

        Ok(StockMovement {
            id: row.get("id"),
            product_id: row.get("product_id"),
            warehouse_id: row.get("warehouse_id"),
            movement_type: row.get("movement_type"),
            quantity: row.get::<_, f64>("quantity"),
            unit_cost: row.get::<_, Option<f64>>("unit_cost"),
            reference: row.get("reference"),
            notes: row.get("notes"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"),
        })
    }

    pub async fn get_stock_levels(pool: &DbPool) -> AppResult<Vec<StockLevel>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT sl.product_id, p.name as product_name, p.sku, sl.warehouse_id, w.name as warehouse_name,
                    sl.quantity, sl.updated_at
             FROM stock_levels sl
             JOIN products p ON sl.product_id = p.id
             JOIN warehouses w ON sl.warehouse_id = w.id
             ORDER BY p.name, w.name",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| StockLevel {
            product_id: r.get("product_id"),
            product_name: r.get("product_name"),
            sku: r.get("sku"),
            warehouse_id: r.get("warehouse_id"),
            warehouse_name: r.get("warehouse_name"),
            quantity: r.get::<_, f64>("quantity"),
            updated_at: r.get("updated_at"),
        }).collect())
    }
}
