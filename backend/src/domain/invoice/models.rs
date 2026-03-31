use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invoice {
    pub id: Uuid,
    pub invoice_number: String,
    pub invoice_type: String,
    pub customer_id: Option<Uuid>,
    pub invoice_date: NaiveDate,
    pub due_date: Option<NaiveDate>,
    pub status: String,
    pub subtotal: f64,
    pub tax_amount: f64,
    pub total: f64,
    pub paid_amount: f64,
    pub currency: String,
    pub notes: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub lines: Vec<InvoiceLine>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvoiceLine {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub tax_rate: f64,
    pub tax_amount: f64,
    pub line_total: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateInvoiceRequest {
    pub invoice_type: String,
    pub customer_id: Option<Uuid>,
    pub invoice_date: NaiveDate,
    pub due_date: Option<NaiveDate>,
    pub currency: Option<String>,
    pub notes: Option<String>,
    pub lines: Vec<CreateInvoiceLineRequest>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInvoiceLineRequest {
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub tax_rate: Option<f64>,
}
