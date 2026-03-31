use uuid::Uuid;
use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use super::models::*;

pub struct BankService;

impl BankService {
    pub async fn list_accounts(pool: &DbPool) -> AppResult<Vec<BankAccount>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, account_number, bank_name, branch, currency, balance, account_id, is_active, created_at, updated_at
             FROM bank_accounts ORDER BY bank_name",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| BankAccount {
            id: r.get("id"), account_number: r.get("account_number"),
            bank_name: r.get("bank_name"), branch: r.get("branch"),
            currency: r.get("currency"), balance: r.get::<_, f64>("balance"),
            account_id: r.get("account_id"), is_active: r.get("is_active"),
            created_at: r.get("created_at"), updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn create_account(pool: &DbPool, req: &CreateBankAccountRequest) -> AppResult<BankAccount> {
        let client = pool.get().await?;
        let currency = req.currency.as_deref().unwrap_or("USD");
        let row = client.query_one(
            "INSERT INTO bank_accounts (account_number, bank_name, branch, currency, account_id)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, account_number, bank_name, branch, currency, balance, account_id, is_active, created_at, updated_at",
            &[&req.account_number, &req.bank_name, &req.branch, &currency, &req.account_id],
        ).await?;
        Ok(BankAccount {
            id: row.get("id"), account_number: row.get("account_number"),
            bank_name: row.get("bank_name"), branch: row.get("branch"),
            currency: row.get("currency"), balance: row.get::<_, f64>("balance"),
            account_id: row.get("account_id"), is_active: row.get("is_active"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
        })
    }

    pub async fn list_transactions(pool: &DbPool, account_id: Uuid) -> AppResult<Vec<BankTransaction>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, bank_account_id, transaction_type, amount, balance_after, reference, description,
                    transaction_date, journal_entry_id, created_by, created_at
             FROM bank_transactions WHERE bank_account_id = $1 ORDER BY transaction_date DESC",
            &[&account_id],
        ).await?;
        Ok(rows.iter().map(|r| BankTransaction {
            id: r.get("id"), bank_account_id: r.get("bank_account_id"),
            transaction_type: r.get("transaction_type"), amount: r.get::<_, f64>("amount"),
            balance_after: r.get::<_, f64>("balance_after"),
            reference: r.get("reference"), description: r.get("description"),
            transaction_date: r.get("transaction_date"),
            journal_entry_id: r.get("journal_entry_id"), created_by: r.get("created_by"),
            created_at: r.get("created_at"),
        }).collect())
    }

    pub async fn create_transaction(pool: &DbPool, req: &CreateBankTransactionRequest, user_id: Uuid) -> AppResult<BankTransaction> {
        let mut client = pool.get().await?;
        let tx = client.transaction().await?;

        let acc_row = tx.query_opt(
            "SELECT balance FROM bank_accounts WHERE id = $1 FOR UPDATE",
            &[&req.bank_account_id],
        ).await?.ok_or_else(|| AppError::NotFound("Bank account not found".to_string()))?;

        let current_balance: f64 = acc_row.get::<_, f64>("balance");
        let new_balance = match req.transaction_type.as_str() {
            "deposit" | "credit" => current_balance + req.amount,
            "withdrawal" | "debit" => current_balance - req.amount,
            _ => current_balance + req.amount,
        };

        tx.execute(
            "UPDATE bank_accounts SET balance = $1, updated_at = NOW() WHERE id = $2",
            &[&new_balance, &req.bank_account_id],
        ).await?;

        let row = tx.query_one(
            "INSERT INTO bank_transactions (bank_account_id, transaction_type, amount, balance_after, reference, description, transaction_date, created_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING id, bank_account_id, transaction_type, amount, balance_after, reference, description,
                       transaction_date, journal_entry_id, created_by, created_at",
            &[&req.bank_account_id, &req.transaction_type, &req.amount, &new_balance,
              &req.reference, &req.description, &req.transaction_date, &user_id],
        ).await?;

        tx.commit().await?;

        Ok(BankTransaction {
            id: row.get("id"), bank_account_id: row.get("bank_account_id"),
            transaction_type: row.get("transaction_type"), amount: row.get::<_, f64>("amount"),
            balance_after: row.get::<_, f64>("balance_after"),
            reference: row.get("reference"), description: row.get("description"),
            transaction_date: row.get("transaction_date"),
            journal_entry_id: row.get("journal_entry_id"), created_by: row.get("created_by"),
            created_at: row.get("created_at"),
        })
    }
}
