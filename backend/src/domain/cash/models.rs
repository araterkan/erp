use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashRegister {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub currency: String,
    pub balance: f64,
    pub account_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCashRegisterRequest {
    pub name: String,
    pub code: String,
    pub currency: Option<String>,
    pub account_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashTransaction {
    pub id: Uuid,
    pub cash_register_id: Uuid,
    pub transaction_type: String,
    pub amount: f64,
    pub balance_after: f64,
    pub reference: Option<String>,
    pub description: Option<String>,
    pub transaction_date: NaiveDate,
    pub journal_entry_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCashTransactionRequest {
    pub cash_register_id: Uuid,
    pub transaction_type: String,
    pub amount: f64,
    pub reference: Option<String>,
    pub description: Option<String>,
    pub transaction_date: NaiveDate,
}
