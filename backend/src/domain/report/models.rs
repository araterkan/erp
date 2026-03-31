use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_customers: i64,
    pub total_products: i64,
    pub total_employees: i64,
    pub total_invoices: i64,
    pub total_invoice_amount: f64,
    pub total_paid_amount: f64,
    pub outstanding_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesReport {
    pub period: String,
    pub total_sales: f64,
    pub total_tax: f64,
    pub invoice_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryReport {
    pub product_id: String,
    pub product_name: String,
    pub sku: String,
    pub total_quantity: f64,
    pub total_value: f64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ReportDateRange {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
