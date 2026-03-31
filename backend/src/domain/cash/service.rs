use uuid::Uuid;
use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use super::models::*;

pub struct CashService;

impl CashService {
    pub async fn list_registers(pool: &DbPool) -> AppResult<Vec<CashRegister>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, name, code, currency, balance, account_id, is_active, created_at, updated_at
             FROM cash_registers ORDER BY name",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| CashRegister {
            id: r.get("id"), name: r.get("name"), code: r.get("code"),
            currency: r.get("currency"), balance: r.get::<_, f64>("balance"),
            account_id: r.get("account_id"), is_active: r.get("is_active"),
            created_at: r.get("created_at"), updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn create_register(pool: &DbPool, req: &CreateCashRegisterRequest) -> AppResult<CashRegister> {
        let client = pool.get().await?;
        let currency = req.currency.as_deref().unwrap_or("USD");
        let row = client.query_one(
            "INSERT INTO cash_registers (name, code, currency, account_id) VALUES ($1, $2, $3, $4)
             RETURNING id, name, code, currency, balance, account_id, is_active, created_at, updated_at",
            &[&req.name, &req.code, &currency, &req.account_id],
        ).await?;
        Ok(CashRegister {
            id: row.get("id"), name: row.get("name"), code: row.get("code"),
            currency: row.get("currency"), balance: row.get::<_, f64>("balance"),
            account_id: row.get("account_id"), is_active: row.get("is_active"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
        })
    }

    pub async fn list_transactions(pool: &DbPool, register_id: Uuid) -> AppResult<Vec<CashTransaction>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, cash_register_id, transaction_type, amount, balance_after, reference, description,
                    transaction_date, journal_entry_id, created_by, created_at
             FROM cash_transactions WHERE cash_register_id = $1 ORDER BY transaction_date DESC",
            &[&register_id],
        ).await?;
        Ok(rows.iter().map(|r| CashTransaction {
            id: r.get("id"), cash_register_id: r.get("cash_register_id"),
            transaction_type: r.get("transaction_type"), amount: r.get::<_, f64>("amount"),
            balance_after: r.get::<_, f64>("balance_after"),
            reference: r.get("reference"), description: r.get("description"),
            transaction_date: r.get("transaction_date"),
            journal_entry_id: r.get("journal_entry_id"), created_by: r.get("created_by"),
            created_at: r.get("created_at"),
        }).collect())
    }

    pub async fn create_transaction(pool: &DbPool, req: &CreateCashTransactionRequest, user_id: Uuid) -> AppResult<CashTransaction> {
        let mut client = pool.get().await?;
        let tx = client.transaction().await?;

        let reg_row = tx.query_opt(
            "SELECT balance FROM cash_registers WHERE id = $1 FOR UPDATE",
            &[&req.cash_register_id],
        ).await?.ok_or_else(|| AppError::NotFound("Cash register not found".to_string()))?;

        let current_balance: f64 = reg_row.get::<_, f64>("balance");
        let new_balance = match req.transaction_type.as_str() {
            "in" | "receipt" => current_balance + req.amount,
            "out" | "payment" => current_balance - req.amount,
            _ => current_balance + req.amount,
        };

        tx.execute(
            "UPDATE cash_registers SET balance = $1, updated_at = NOW() WHERE id = $2",
            &[&new_balance, &req.cash_register_id],
        ).await?;

        let row = tx.query_one(
            "INSERT INTO cash_transactions (cash_register_id, transaction_type, amount, balance_after, reference, description, transaction_date, created_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING id, cash_register_id, transaction_type, amount, balance_after, reference, description,
                       transaction_date, journal_entry_id, created_by, created_at",
            &[&req.cash_register_id, &req.transaction_type, &req.amount, &new_balance,
              &req.reference, &req.description, &req.transaction_date, &user_id],
        ).await?;

        tx.commit().await?;

        Ok(CashTransaction {
            id: row.get("id"), cash_register_id: row.get("cash_register_id"),
            transaction_type: row.get("transaction_type"), amount: row.get::<_, f64>("amount"),
            balance_after: row.get::<_, f64>("balance_after"),
            reference: row.get("reference"), description: row.get("description"),
            transaction_date: row.get("transaction_date"),
            journal_entry_id: row.get("journal_entry_id"), created_by: row.get("created_by"),
            created_at: row.get("created_at"),
        })
    }
}
