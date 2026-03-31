use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub is_active: bool,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UpdateAccountRequest {
    pub name: Option<String>,
    pub account_type: Option<String>,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JournalEntry {
    pub id: Uuid,
    pub entry_number: String,
    pub description: String,
    pub reference: Option<String>,
    pub entry_date: NaiveDate,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub lines: Vec<JournalLine>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JournalLine {
    pub id: Uuid,
    pub journal_entry_id: Uuid,
    pub account_id: Uuid,
    pub account_name: Option<String>,
    pub debit: f64,
    pub credit: f64,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJournalEntryRequest {
    pub description: String,
    pub reference: Option<String>,
    pub entry_date: NaiveDate,
    pub lines: Vec<CreateJournalLineRequest>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJournalLineRequest {
    pub account_id: Uuid,
    pub debit: f64,
    pub credit: f64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TrialBalance {
    pub accounts: Vec<TrialBalanceRow>,
    pub total_debit: f64,
    pub total_credit: f64,
}

#[derive(Debug, Serialize)]
pub struct TrialBalanceRow {
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub total_debit: f64,
    pub total_credit: f64,
    pub balance: f64,
}
