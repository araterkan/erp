use uuid::Uuid;
use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use super::models::*;

pub struct InvoiceService;

impl InvoiceService {
    pub async fn list_invoices(pool: &DbPool) -> AppResult<Vec<Invoice>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, invoice_number, invoice_type, customer_id, invoice_date, due_date, status,
                    subtotal, tax_amount, total, paid_amount, currency, notes, journal_entry_id, created_by, created_at, updated_at
             FROM invoices ORDER BY invoice_date DESC LIMIT 100",
            &[],
        ).await?;

        let mut invoices = Vec::new();
        for row in &rows {
            let inv_id: Uuid = row.get("id");
            let line_rows = client.query(
                "SELECT id, invoice_id, product_id, description, quantity, unit_price, tax_rate, tax_amount, line_total
                 FROM invoice_lines WHERE invoice_id = $1",
                &[&inv_id],
            ).await?;

            let lines: Vec<InvoiceLine> = line_rows.iter().map(|r| InvoiceLine {
                id: r.get("id"), invoice_id: r.get("invoice_id"),
                product_id: r.get("product_id"), description: r.get("description"),
                quantity: r.get::<_, f64>("quantity"), unit_price: r.get::<_, f64>("unit_price"),
                tax_rate: r.get::<_, f64>("tax_rate"), tax_amount: r.get::<_, f64>("tax_amount"),
                line_total: r.get::<_, f64>("line_total"),
            }).collect();

            invoices.push(Invoice {
                id: inv_id, invoice_number: row.get("invoice_number"),
                invoice_type: row.get("invoice_type"), customer_id: row.get("customer_id"),
                invoice_date: row.get("invoice_date"), due_date: row.get("due_date"),
                status: row.get("status"), subtotal: row.get::<_, f64>("subtotal"),
                tax_amount: row.get::<_, f64>("tax_amount"), total: row.get::<_, f64>("total"),
                paid_amount: row.get::<_, f64>("paid_amount"), currency: row.get("currency"),
                notes: row.get("notes"), journal_entry_id: row.get("journal_entry_id"),
                created_by: row.get("created_by"),
                created_at: row.get("created_at"), updated_at: row.get("updated_at"),
                lines,
            });
        }
        Ok(invoices)
    }

    pub async fn get_invoice(pool: &DbPool, id: Uuid) -> AppResult<Invoice> {
        let client = pool.get().await?;
        let row = client.query_opt(
            "SELECT id, invoice_number, invoice_type, customer_id, invoice_date, due_date, status,
                    subtotal, tax_amount, total, paid_amount, currency, notes, journal_entry_id, created_by, created_at, updated_at
             FROM invoices WHERE id = $1",
            &[&id],
        ).await?.ok_or_else(|| AppError::NotFound("Invoice not found".to_string()))?;

        let inv_id: Uuid = row.get("id");
        let line_rows = client.query(
            "SELECT id, invoice_id, product_id, description, quantity, unit_price, tax_rate, tax_amount, line_total
             FROM invoice_lines WHERE invoice_id = $1",
            &[&inv_id],
        ).await?;

        let lines: Vec<InvoiceLine> = line_rows.iter().map(|r| InvoiceLine {
            id: r.get("id"), invoice_id: r.get("invoice_id"),
            product_id: r.get("product_id"), description: r.get("description"),
            quantity: r.get::<_, f64>("quantity"), unit_price: r.get::<_, f64>("unit_price"),
            tax_rate: r.get::<_, f64>("tax_rate"), tax_amount: r.get::<_, f64>("tax_amount"),
            line_total: r.get::<_, f64>("line_total"),
        }).collect();

        Ok(Invoice {
            id: inv_id, invoice_number: row.get("invoice_number"),
            invoice_type: row.get("invoice_type"), customer_id: row.get("customer_id"),
            invoice_date: row.get("invoice_date"), due_date: row.get("due_date"),
            status: row.get("status"), subtotal: row.get::<_, f64>("subtotal"),
            tax_amount: row.get::<_, f64>("tax_amount"), total: row.get::<_, f64>("total"),
            paid_amount: row.get::<_, f64>("paid_amount"), currency: row.get("currency"),
            notes: row.get("notes"), journal_entry_id: row.get("journal_entry_id"),
            created_by: row.get("created_by"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
            lines,
        })
    }

    pub async fn create_invoice(pool: &DbPool, req: &CreateInvoiceRequest, user_id: Uuid) -> AppResult<Invoice> {
        let mut client = pool.get().await?;
        let tx = client.transaction().await?;

        let count_row = tx.query_one("SELECT COUNT(*) + 1 as num FROM invoices", &[]).await?;
        let num: i64 = count_row.get("num");
        let prefix = match req.invoice_type.as_str() {
            "purchase" => "PI",
            _ => "SI",
        };
        let invoice_number = format!("{}-{:08}", prefix, num);
        let currency = req.currency.as_deref().unwrap_or("USD");

        let mut subtotal = 0.0f64;
        let mut tax_total = 0.0f64;
        let mut line_data = Vec::new();

        for line in &req.lines {
            let tax_rate = line.tax_rate.unwrap_or(0.0);
            let line_subtotal = line.quantity * line.unit_price;
            let tax_amount = line_subtotal * tax_rate / 100.0;
            let line_total = line_subtotal + tax_amount;
            subtotal += line_subtotal;
            tax_total += tax_amount;
            line_data.push((line, tax_rate, tax_amount, line_total));
        }

        let total = subtotal + tax_total;

        let inv_row = tx.query_one(
            "INSERT INTO invoices (invoice_number, invoice_type, customer_id, invoice_date, due_date,
                                   subtotal, tax_amount, total, currency, notes, created_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             RETURNING id, invoice_number, invoice_type, customer_id, invoice_date, due_date, status,
                       subtotal, tax_amount, total, paid_amount, currency, notes, journal_entry_id, created_by, created_at, updated_at",
            &[&invoice_number, &req.invoice_type, &req.customer_id, &req.invoice_date, &req.due_date,
              &subtotal, &tax_total, &total, &currency, &req.notes, &user_id],
        ).await?;

        let inv_id: Uuid = inv_row.get("id");
        let mut lines = Vec::new();

        for (line, tax_rate, tax_amount, line_total) in &line_data {
            let l_row = tx.query_one(
                "INSERT INTO invoice_lines (invoice_id, product_id, description, quantity, unit_price, tax_rate, tax_amount, line_total)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                 RETURNING id, invoice_id, product_id, description, quantity, unit_price, tax_rate, tax_amount, line_total",
                &[&inv_id, &line.product_id, &line.description, &line.quantity, &line.unit_price, tax_rate, tax_amount, line_total],
            ).await?;
            lines.push(InvoiceLine {
                id: l_row.get("id"), invoice_id: l_row.get("invoice_id"),
                product_id: l_row.get("product_id"), description: l_row.get("description"),
                quantity: l_row.get::<_, f64>("quantity"), unit_price: l_row.get::<_, f64>("unit_price"),
                tax_rate: l_row.get::<_, f64>("tax_rate"), tax_amount: l_row.get::<_, f64>("tax_amount"),
                line_total: l_row.get::<_, f64>("line_total"),
            });
        }

        tx.commit().await?;

        Ok(Invoice {
            id: inv_id, invoice_number: inv_row.get("invoice_number"),
            invoice_type: inv_row.get("invoice_type"), customer_id: inv_row.get("customer_id"),
            invoice_date: inv_row.get("invoice_date"), due_date: inv_row.get("due_date"),
            status: inv_row.get("status"), subtotal: inv_row.get::<_, f64>("subtotal"),
            tax_amount: inv_row.get::<_, f64>("tax_amount"), total: inv_row.get::<_, f64>("total"),
            paid_amount: inv_row.get::<_, f64>("paid_amount"), currency: inv_row.get("currency"),
            notes: inv_row.get("notes"), journal_entry_id: inv_row.get("journal_entry_id"),
            created_by: inv_row.get("created_by"),
            created_at: inv_row.get("created_at"), updated_at: inv_row.get("updated_at"),
            lines,
        })
    }
}
