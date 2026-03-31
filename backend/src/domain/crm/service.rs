use uuid::Uuid;
use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use super::models::*;

pub struct CrmService;

impl CrmService {
    pub async fn list_customers(pool: &DbPool) -> AppResult<Vec<Customer>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, code, name, email, phone, address, city, country, tax_number, credit_limit, is_active, created_at, updated_at
             FROM customers ORDER BY name",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| Customer {
            id: r.get("id"), code: r.get("code"), name: r.get("name"),
            email: r.get("email"), phone: r.get("phone"), address: r.get("address"),
            city: r.get("city"), country: r.get("country"), tax_number: r.get("tax_number"),
            credit_limit: r.get::<_, Option<f64>>("credit_limit"),
            is_active: r.get("is_active"),
            created_at: r.get("created_at"), updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn get_customer(pool: &DbPool, id: Uuid) -> AppResult<Customer> {
        let client = pool.get().await?;
        let row = client.query_opt(
            "SELECT id, code, name, email, phone, address, city, country, tax_number, credit_limit, is_active, created_at, updated_at
             FROM customers WHERE id = $1",
            &[&id],
        ).await?.ok_or_else(|| AppError::NotFound("Customer not found".to_string()))?;
        Ok(Customer {
            id: row.get("id"), code: row.get("code"), name: row.get("name"),
            email: row.get("email"), phone: row.get("phone"), address: row.get("address"),
            city: row.get("city"), country: row.get("country"), tax_number: row.get("tax_number"),
            credit_limit: row.get::<_, Option<f64>>("credit_limit"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
        })
    }

    pub async fn create_customer(pool: &DbPool, req: &CreateCustomerRequest) -> AppResult<Customer> {
        let client = pool.get().await?;
        let count_row = client.query_one("SELECT COUNT(*) + 1 as num FROM customers", &[]).await?;
        let num: i64 = count_row.get("num");
        let code = format!("CUST-{:06}", num);

        let row = client.query_one(
            "INSERT INTO customers (code, name, email, phone, address, city, country, tax_number, credit_limit)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING id, code, name, email, phone, address, city, country, tax_number, credit_limit, is_active, created_at, updated_at",
            &[&code, &req.name, &req.email, &req.phone, &req.address, &req.city, &req.country, &req.tax_number, &req.credit_limit],
        ).await?;
        Ok(Customer {
            id: row.get("id"), code: row.get("code"), name: row.get("name"),
            email: row.get("email"), phone: row.get("phone"), address: row.get("address"),
            city: row.get("city"), country: row.get("country"), tax_number: row.get("tax_number"),
            credit_limit: row.get::<_, Option<f64>>("credit_limit"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
        })
    }

    pub async fn list_contacts(pool: &DbPool) -> AppResult<Vec<Contact>> {
        let client = pool.get().await?;
        let rows = client.query(
            "SELECT id, customer_id, first_name, last_name, email, phone, position, is_primary, created_at, updated_at
             FROM contacts ORDER BY last_name, first_name",
            &[],
        ).await?;
        Ok(rows.iter().map(|r| Contact {
            id: r.get("id"), customer_id: r.get("customer_id"),
            first_name: r.get("first_name"), last_name: r.get("last_name"),
            email: r.get("email"), phone: r.get("phone"), position: r.get("position"),
            is_primary: r.get("is_primary"),
            created_at: r.get("created_at"), updated_at: r.get("updated_at"),
        }).collect())
    }

    pub async fn create_contact(pool: &DbPool, req: &CreateContactRequest) -> AppResult<Contact> {
        let client = pool.get().await?;
        let is_primary = req.is_primary.unwrap_or(false);
        let row = client.query_one(
            "INSERT INTO contacts (customer_id, first_name, last_name, email, phone, position, is_primary)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING id, customer_id, first_name, last_name, email, phone, position, is_primary, created_at, updated_at",
            &[&req.customer_id, &req.first_name, &req.last_name, &req.email, &req.phone, &req.position, &is_primary],
        ).await?;
        Ok(Contact {
            id: row.get("id"), customer_id: row.get("customer_id"),
            first_name: row.get("first_name"), last_name: row.get("last_name"),
            email: row.get("email"), phone: row.get("phone"), position: row.get("position"),
            is_primary: row.get("is_primary"),
            created_at: row.get("created_at"), updated_at: row.get("updated_at"),
        })
    }
}
