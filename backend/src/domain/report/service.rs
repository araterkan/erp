use crate::db::DbPool;
use crate::error::AppResult;
use super::models::*;

pub struct ReportService;

impl ReportService {
    pub async fn get_dashboard_stats(pool: &DbPool) -> AppResult<DashboardStats> {
        let client = pool.get().await?;

        let cust_row = client.query_one("SELECT COUNT(*) as cnt FROM current_accounts WHERE is_active = true", &[]).await?;
        let prod_row = client.query_one("SELECT COUNT(*) as cnt FROM products WHERE is_active = true", &[]).await?;
        let emp_row = client.query_one("SELECT COUNT(*) as cnt FROM employees WHERE is_active = true", &[]).await?;
        let inv_row = client.query_one(
            "SELECT COUNT(*) as cnt, COALESCE(SUM(total_amount), 0.0) as total_amt, COALESCE(SUM(paid_amount), 0.0) as paid_amt
             FROM invoices WHERE invoice_type = 'sale'",
            &[],
        ).await?;

        let total_invoice_amount: f64 = inv_row.get::<_, f64>("total_amt");
        let total_paid_amount: f64 = inv_row.get::<_, f64>("paid_amt");

        Ok(DashboardStats {
            total_customers: cust_row.get::<_, i64>("cnt"),
            total_products: prod_row.get::<_, i64>("cnt"),
            total_employees: emp_row.get::<_, i64>("cnt"),
            total_invoices: inv_row.get::<_, i64>("cnt"),
            total_invoice_amount,
            total_paid_amount,
            outstanding_amount: total_invoice_amount - total_paid_amount,
        })
    }

    pub async fn get_sales_report(pool: &DbPool) -> AppResult<Vec<SalesReport>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT TO_CHAR(invoice_date, 'YYYY-MM') as period,
                    COALESCE(SUM(total_amount), 0.0) as total_sales,
                    COALESCE(SUM(vat_amount), 0.0) as total_tax,
                    COUNT(*) as invoice_count
             FROM invoices
             WHERE invoice_type = 'sale' AND status != 'cancelled'
             GROUP BY TO_CHAR(invoice_date, 'YYYY-MM')
             ORDER BY period DESC
             LIMIT 12",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| SalesReport {
            period: r.get("period"),
            total_sales: r.get::<_, f64>("total_sales"),
            total_tax: r.get::<_, f64>("total_tax"),
            invoice_count: r.get::<_, i64>("invoice_count"),
        }).collect())
    }

    pub async fn get_inventory_report(pool: &DbPool) -> AppResult<Vec<InventoryReport>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT p.id::TEXT as product_id, p.name as product_name, p.sku,
                    COALESCE(SUM(sl.quantity), 0.0) as total_quantity,
                    COALESCE(SUM(sl.quantity * COALESCE(p.purchase_price, 0)), 0.0) as total_value
             FROM products p
             LEFT JOIN stock_levels sl ON p.id = sl.product_id
             WHERE p.is_active = true
             GROUP BY p.id, p.name, p.sku
             ORDER BY p.name",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| InventoryReport {
            product_id: r.get("product_id"),
            product_name: r.get("product_name"),
            sku: r.get("sku"),
            total_quantity: r.get::<_, f64>("total_quantity"),
            total_value: r.get::<_, f64>("total_value"),
        }).collect())
    }
}
