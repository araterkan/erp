use uuid::Uuid;

use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use super::models::{
    Account, CreateAccountRequest, CreateJournalEntryRequest, JournalEntry, JournalLine, TrialBalance, TrialBalanceRow,
};

pub struct FinanceService;

impl FinanceService {
    pub async fn list_accounts(pool: &DbPool) -> AppResult<Vec<Account>> {
        let client = pool.get().await?;
        let rows = client
            .query(
                "SELECT id, code, name, account_type, parent_id, is_active, description, created_at, updated_at
                 FROM accounts ORDER BY code",
                &[],
            )
            .await?;

        Ok(rows.iter().map(|r| Account {
            id: r.get("id"),
            code: r.get("code"),
            name: r.get("name"),
            account_type: r.get("account_type"),
            parent_id: r.get("parent_id"),
            is_active: r.get("is_active"),
            description: r.get("description"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn get_account(pool: &DbPool, id: Uuid) -> AppResult<Account> {
        let client = pool.get().await?;
        let row = client
            .query_opt(
                "SELECT id, code, name, account_type, parent_id, is_active, description, created_at, updated_at
                 FROM accounts WHERE id = $1",
                &[&id],
            )
            .await?
            .ok_or_else(|| AppError::NotFound("Account not found".to_string()))?;

        Ok(Account {
            id: row.get("id"),
            code: row.get("code"),
            name: row.get("name"),
            account_type: row.get("account_type"),
            parent_id: row.get("parent_id"),
            is_active: row.get("is_active"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn create_account(pool: &DbPool, req: &CreateAccountRequest) -> AppResult<Account> {
        let client = pool.get().await?;
        let row = client
            .query_one(
                "INSERT INTO accounts (code, name, account_type, parent_id, description)
                 VALUES ($1, $2, $3, $4, $5)
                 RETURNING id, code, name, account_type, parent_id, is_active, description, created_at, updated_at",
                &[&req.code, &req.name, &req.account_type, &req.parent_id, &req.description],
            )
            .await?;

        Ok(Account {
            id: row.get("id"),
            code: row.get("code"),
            name: row.get("name"),
            account_type: row.get("account_type"),
            parent_id: row.get("parent_id"),
            is_active: row.get("is_active"),
            description: row.get("description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn list_journal_entries(pool: &DbPool) -> AppResult<Vec<JournalEntry>> {
        let client = pool.get().await?;
        let rows = client
            .query(
                "SELECT id, entry_number, description, reference, entry_date, status, created_by, created_at, updated_at
                 FROM journal_entries ORDER BY entry_date DESC, created_at DESC LIMIT 100",
                &[],
            )
            .await?;

        let mut entries = Vec::new();
        for row in &rows {
            let entry_id: Uuid = row.get("id");
            let line_rows = client
                .query(
                    "SELECT jl.id, jl.journal_entry_id, jl.account_id, a.name as account_name,
                            jl.debit, jl.credit, jl.description
                     FROM journal_lines jl
                     LEFT JOIN accounts a ON jl.account_id = a.id
                     WHERE jl.journal_entry_id = $1",
                    &[&entry_id],
                )
                .await?;

            let lines: Vec<JournalLine> = line_rows.iter().map(|r| JournalLine {
                id: r.get("id"),
                journal_entry_id: r.get("journal_entry_id"),
                account_id: r.get("account_id"),
                account_name: r.get("account_name"),
                debit: r.get::<_, f64>("debit"),
                credit: r.get::<_, f64>("credit"),
                description: r.get("description"),
            }).collect();

            entries.push(JournalEntry {
                id: entry_id,
                entry_number: row.get("entry_number"),
                description: row.get("description"),
                reference: row.get("reference"),
                entry_date: row.get("entry_date"),
                status: row.get("status"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                lines,
            });
        }

        Ok(entries)
    }

    pub async fn create_journal_entry(
        pool: &DbPool,
        req: &CreateJournalEntryRequest,
        user_id: Uuid,
    ) -> AppResult<JournalEntry> {
        let total_debit: f64 = req.lines.iter().map(|l| l.debit).sum();
        let total_credit: f64 = req.lines.iter().map(|l| l.credit).sum();

        if (total_debit - total_credit).abs() > 0.001 {
            return Err(AppError::BadRequest(
                "Journal entry is not balanced: debits must equal credits".to_string(),
            ));
        }

        let mut client = pool.get().await?;
        let tx = client.transaction().await?;

        let count_row = tx
            .query_one("SELECT COUNT(*) + 1 as num FROM journal_entries", &[])
            .await?;
        let num: i64 = count_row.get("num");
        let entry_number = format!("JE-{:08}", num);

        let entry_row = tx
            .query_one(
                "INSERT INTO journal_entries (entry_number, description, reference, entry_date, created_by)
                 VALUES ($1, $2, $3, $4, $5)
                 RETURNING id, entry_number, description, reference, entry_date, status, created_by, created_at, updated_at",
                &[&entry_number, &req.description, &req.reference, &req.entry_date, &user_id],
            )
            .await?;

        let entry_id: Uuid = entry_row.get("id");
        let mut lines = Vec::new();

        for line in &req.lines {
            let line_row = tx
                .query_one(
                    "INSERT INTO journal_lines (journal_entry_id, account_id, debit, credit, description)
                     VALUES ($1, $2, $3, $4, $5)
                     RETURNING id, journal_entry_id, account_id, debit, credit, description",
                    &[&entry_id, &line.account_id, &line.debit, &line.credit, &line.description],
                )
                .await?;

            lines.push(JournalLine {
                id: line_row.get("id"),
                journal_entry_id: line_row.get("journal_entry_id"),
                account_id: line_row.get("account_id"),
                account_name: None,
                debit: line_row.get::<_, f64>("debit"),
                credit: line_row.get::<_, f64>("credit"),
                description: line_row.get("description"),
            });
        }

        tx.commit().await?;

        Ok(JournalEntry {
            id: entry_id,
            entry_number: entry_row.get("entry_number"),
            description: entry_row.get("description"),
            reference: entry_row.get("reference"),
            entry_date: entry_row.get("entry_date"),
            status: entry_row.get("status"),
            created_by: entry_row.get("created_by"),
            created_at: entry_row.get("created_at"),
            updated_at: entry_row.get("updated_at"),
            lines,
        })
    }

    pub async fn get_trial_balance(pool: &DbPool) -> AppResult<TrialBalance> {
        let client = pool.get().await?;
        let rows = client
            .query(
                "SELECT a.code as account_code, a.name as account_name, a.account_type,
                        COALESCE(SUM(jl.debit), 0.0) as total_debit,
                        COALESCE(SUM(jl.credit), 0.0) as total_credit
                 FROM accounts a
                 LEFT JOIN journal_lines jl ON a.id = jl.account_id
                 LEFT JOIN journal_entries je ON jl.journal_entry_id = je.id AND je.status = 'posted'
                 WHERE a.is_active = true
                 GROUP BY a.id, a.code, a.name, a.account_type
                 ORDER BY a.code",
                &[],
            )
            .await?;

        let mut total_debit = 0.0f64;
        let mut total_credit = 0.0f64;
        let accounts: Vec<TrialBalanceRow> = rows.iter().map(|r| {
            let td: f64 = r.get::<_, f64>("total_debit");
            let tc: f64 = r.get::<_, f64>("total_credit");
            total_debit += td;
            total_credit += tc;
            TrialBalanceRow {
                account_code: r.get("account_code"),
                account_name: r.get("account_name"),
                account_type: r.get("account_type"),
                total_debit: td,
                total_credit: tc,
                balance: td - tc,
            }
        }).collect();

        Ok(TrialBalance { accounts, total_debit, total_credit })
    }
}
