use deadpool_postgres::{Config as PgConfig, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use crate::config::Config;

pub type DbPool = Pool;

pub fn create_pool(config: &Config) -> anyhow::Result<DbPool> {
    let mut pg_config = PgConfig::new();
    pg_config.host = Some(config.db_host.clone());
    pg_config.port = Some(config.db_port);
    pg_config.dbname = Some(config.db_name.clone());
    pg_config.user = Some(config.db_user.clone());
    pg_config.password = Some(config.db_password.clone());
    pg_config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = pg_config.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> anyhow::Result<()> {
    let client = pool.get().await?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username VARCHAR(100) UNIQUE NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            salt TEXT NOT NULL,
            role VARCHAR(50) NOT NULL DEFAULT 'user',
            is_active BOOLEAN NOT NULL DEFAULT true,
            first_name VARCHAR(100),
            last_name VARCHAR(100),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS audit_logs (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID REFERENCES users(id),
            username VARCHAR(100),
            action VARCHAR(255) NOT NULL,
            resource VARCHAR(255),
            resource_id VARCHAR(255),
            method VARCHAR(10),
            path TEXT,
            status_code INTEGER,
            ip_address VARCHAR(50),
            user_agent TEXT,
            request_body TEXT,
            response_time_ms BIGINT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS refresh_tokens (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            token TEXT NOT NULL UNIQUE,
            expires_at TIMESTAMPTZ NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS accounts (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            code VARCHAR(20) UNIQUE NOT NULL,
            name VARCHAR(255) NOT NULL,
            account_type VARCHAR(50) NOT NULL,
            parent_id UUID REFERENCES accounts(id),
            is_active BOOLEAN NOT NULL DEFAULT true,
            description TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS journal_entries (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            entry_number VARCHAR(50) UNIQUE NOT NULL,
            description TEXT NOT NULL,
            reference VARCHAR(255),
            entry_date DATE NOT NULL,
            status VARCHAR(20) NOT NULL DEFAULT 'draft',
            created_by UUID REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS journal_lines (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            journal_entry_id UUID NOT NULL REFERENCES journal_entries(id) ON DELETE CASCADE,
            account_id UUID NOT NULL REFERENCES accounts(id),
            debit NUMERIC(20,4) NOT NULL DEFAULT 0,
            credit NUMERIC(20,4) NOT NULL DEFAULT 0,
            description TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS warehouses (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            code VARCHAR(20) UNIQUE NOT NULL,
            name VARCHAR(255) NOT NULL,
            address TEXT,
            is_active BOOLEAN NOT NULL DEFAULT true,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS product_categories (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL,
            parent_id UUID REFERENCES product_categories(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS products (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            sku VARCHAR(100) UNIQUE NOT NULL,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            category_id UUID REFERENCES product_categories(id),
            unit VARCHAR(50) NOT NULL DEFAULT 'pcs',
            purchase_price NUMERIC(20,4),
            sale_price NUMERIC(20,4),
            tax_rate NUMERIC(5,2) NOT NULL DEFAULT 0,
            is_active BOOLEAN NOT NULL DEFAULT true,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS stock_movements (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            product_id UUID NOT NULL REFERENCES products(id),
            warehouse_id UUID NOT NULL REFERENCES warehouses(id),
            movement_type VARCHAR(50) NOT NULL,
            quantity NUMERIC(20,4) NOT NULL,
            unit_cost NUMERIC(20,4),
            reference VARCHAR(255),
            notes TEXT,
            created_by UUID REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS stock_levels (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            product_id UUID NOT NULL REFERENCES products(id),
            warehouse_id UUID NOT NULL REFERENCES warehouses(id),
            quantity NUMERIC(20,4) NOT NULL DEFAULT 0,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            UNIQUE(product_id, warehouse_id)
        );

        CREATE TABLE IF NOT EXISTS departments (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL,
            code VARCHAR(20) UNIQUE NOT NULL,
            parent_id UUID REFERENCES departments(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS employees (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            employee_number VARCHAR(50) UNIQUE NOT NULL,
            user_id UUID REFERENCES users(id),
            first_name VARCHAR(100) NOT NULL,
            last_name VARCHAR(100) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            phone VARCHAR(50),
            department_id UUID REFERENCES departments(id),
            position VARCHAR(100),
            hire_date DATE NOT NULL,
            termination_date DATE,
            base_salary NUMERIC(20,4),
            is_active BOOLEAN NOT NULL DEFAULT true,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS leave_requests (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            employee_id UUID NOT NULL REFERENCES employees(id),
            leave_type VARCHAR(50) NOT NULL,
            start_date DATE NOT NULL,
            end_date DATE NOT NULL,
            days_count NUMERIC(5,1) NOT NULL,
            reason TEXT,
            status VARCHAR(20) NOT NULL DEFAULT 'pending',
            approved_by UUID REFERENCES employees(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS payroll_runs (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            period_start DATE NOT NULL,
            period_end DATE NOT NULL,
            status VARCHAR(20) NOT NULL DEFAULT 'draft',
            total_gross NUMERIC(20,4) NOT NULL DEFAULT 0,
            total_deductions NUMERIC(20,4) NOT NULL DEFAULT 0,
            total_net NUMERIC(20,4) NOT NULL DEFAULT 0,
            created_by UUID REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS payroll_items (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            payroll_run_id UUID NOT NULL REFERENCES payroll_runs(id) ON DELETE CASCADE,
            employee_id UUID NOT NULL REFERENCES employees(id),
            gross_salary NUMERIC(20,4) NOT NULL,
            deductions NUMERIC(20,4) NOT NULL DEFAULT 0,
            net_salary NUMERIC(20,4) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS customers (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            code VARCHAR(20) UNIQUE NOT NULL,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255),
            phone VARCHAR(50),
            address TEXT,
            city VARCHAR(100),
            country VARCHAR(100),
            tax_number VARCHAR(50),
            credit_limit NUMERIC(20,4) DEFAULT 0,
            is_active BOOLEAN NOT NULL DEFAULT true,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS contacts (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            customer_id UUID REFERENCES customers(id),
            first_name VARCHAR(100) NOT NULL,
            last_name VARCHAR(100) NOT NULL,
            email VARCHAR(255),
            phone VARCHAR(50),
            position VARCHAR(100),
            is_primary BOOLEAN NOT NULL DEFAULT false,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS bank_accounts (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            account_number VARCHAR(100) UNIQUE NOT NULL,
            bank_name VARCHAR(255) NOT NULL,
            branch VARCHAR(255),
            currency VARCHAR(10) NOT NULL DEFAULT 'USD',
            balance NUMERIC(20,4) NOT NULL DEFAULT 0,
            account_id UUID REFERENCES accounts(id),
            is_active BOOLEAN NOT NULL DEFAULT true,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS bank_transactions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            bank_account_id UUID NOT NULL REFERENCES bank_accounts(id),
            transaction_type VARCHAR(50) NOT NULL,
            amount NUMERIC(20,4) NOT NULL,
            balance_after NUMERIC(20,4) NOT NULL,
            reference VARCHAR(255),
            description TEXT,
            transaction_date DATE NOT NULL,
            journal_entry_id UUID REFERENCES journal_entries(id),
            created_by UUID REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS cash_registers (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) NOT NULL,
            code VARCHAR(20) UNIQUE NOT NULL,
            currency VARCHAR(10) NOT NULL DEFAULT 'USD',
            balance NUMERIC(20,4) NOT NULL DEFAULT 0,
            account_id UUID REFERENCES accounts(id),
            is_active BOOLEAN NOT NULL DEFAULT true,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS cash_transactions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            cash_register_id UUID NOT NULL REFERENCES cash_registers(id),
            transaction_type VARCHAR(50) NOT NULL,
            amount NUMERIC(20,4) NOT NULL,
            balance_after NUMERIC(20,4) NOT NULL,
            reference VARCHAR(255),
            description TEXT,
            transaction_date DATE NOT NULL,
            journal_entry_id UUID REFERENCES journal_entries(id),
            created_by UUID REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS invoices (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            invoice_number VARCHAR(50) UNIQUE NOT NULL,
            invoice_type VARCHAR(20) NOT NULL,
            customer_id UUID REFERENCES customers(id),
            invoice_date DATE NOT NULL,
            due_date DATE,
            status VARCHAR(20) NOT NULL DEFAULT 'draft',
            subtotal NUMERIC(20,4) NOT NULL DEFAULT 0,
            tax_amount NUMERIC(20,4) NOT NULL DEFAULT 0,
            total NUMERIC(20,4) NOT NULL DEFAULT 0,
            paid_amount NUMERIC(20,4) NOT NULL DEFAULT 0,
            currency VARCHAR(10) NOT NULL DEFAULT 'USD',
            notes TEXT,
            journal_entry_id UUID REFERENCES journal_entries(id),
            created_by UUID REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS invoice_lines (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            invoice_id UUID NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
            product_id UUID REFERENCES products(id),
            description TEXT NOT NULL,
            quantity NUMERIC(20,4) NOT NULL,
            unit_price NUMERIC(20,4) NOT NULL,
            tax_rate NUMERIC(5,2) NOT NULL DEFAULT 0,
            tax_amount NUMERIC(20,4) NOT NULL DEFAULT 0,
            line_total NUMERIC(20,4) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );

        CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
        CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at);
        CREATE INDEX IF NOT EXISTS idx_journal_lines_entry_id ON journal_lines(journal_entry_id);
        CREATE INDEX IF NOT EXISTS idx_stock_movements_product_id ON stock_movements(product_id);
        CREATE INDEX IF NOT EXISTS idx_invoices_customer_id ON invoices(customer_id);
        CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
    ").await?;

    Ok(())
}

pub async fn seed_admin_user(pool: &DbPool) -> anyhow::Result<()> {
    let client = pool.get().await?;

    let count: i64 = client
        .query_one("SELECT COUNT(*) FROM users WHERE username = 'admin'", &[])
        .await?
        .get(0);

    if count == 0 {
        use sha2::{Sha256, Digest};

        let salt: String = (0..32)
            .map(|_| format!("{:02x}", rand::random::<u8>()))
            .collect();

        let mut hasher = Sha256::new();
        hasher.update(format!("admin{}", salt));
        let password_hash = hex::encode(hasher.finalize());

        client.execute(
            "INSERT INTO users (username, email, password_hash, salt, role, first_name, last_name)
             VALUES ('admin', 'admin@erp.local', $1, $2, 'admin', 'System', 'Administrator')",
            &[&password_hash, &salt],
        ).await?;

        tracing::info!("Admin user seeded successfully");
    }

    Ok(())
}
