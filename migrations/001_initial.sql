-- =============================================================================
-- ERP System - Initial Database Schema
-- PostgreSQL 16+
-- =============================================================================

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- =============================================================================
-- ENUMS
-- =============================================================================

CREATE TYPE user_role AS ENUM ('admin', 'manager', 'accountant', 'hr_manager', 'warehouse_manager', 'viewer');
CREATE TYPE account_type AS ENUM ('asset', 'liability', 'equity', 'revenue', 'expense');
CREATE TYPE movement_type AS ENUM ('in', 'out', 'transfer', 'adjustment');
CREATE TYPE employment_type AS ENUM ('full_time', 'part_time', 'contract', 'intern');
CREATE TYPE leave_status AS ENUM ('pending', 'approved', 'rejected', 'cancelled');
CREATE TYPE leave_type AS ENUM ('annual', 'sick', 'maternity', 'paternity', 'unpaid', 'other');
CREATE TYPE payroll_status AS ENUM ('draft', 'approved', 'paid');
CREATE TYPE current_account_type AS ENUM ('customer', 'supplier', 'both');
CREATE TYPE transaction_type_bank AS ENUM ('credit', 'debit');
CREATE TYPE transaction_type_cash AS ENUM ('in', 'out');
CREATE TYPE invoice_type AS ENUM ('sale', 'purchase');
CREATE TYPE invoice_status AS ENUM ('draft', 'sent', 'paid', 'cancelled');
CREATE TYPE payment_plan_status AS ENUM ('pending', 'paid', 'overdue');
CREATE TYPE check_type AS ENUM ('received', 'issued');
CREATE TYPE check_status AS ENUM ('pending', 'deposited', 'cleared', 'bounced', 'cancelled');
CREATE TYPE serial_status AS ENUM ('available', 'sold', 'reserved', 'defective');
CREATE TYPE task_priority AS ENUM ('low', 'medium', 'high', 'urgent');
CREATE TYPE task_status AS ENUM ('open', 'in_progress', 'completed', 'cancelled');

-- =============================================================================
-- CORE: USERS & AUTH
-- =============================================================================

CREATE TABLE users (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username      VARCHAR(50)  NOT NULL UNIQUE,
    email         VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    salt          VARCHAR(255) NOT NULL,
    role          user_role    NOT NULL DEFAULT 'viewer',
    is_active     BOOLEAN      NOT NULL DEFAULT true,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_username  ON users(username);
CREATE INDEX idx_users_email     ON users(email);
CREATE INDEX idx_users_role      ON users(role);
CREATE INDEX idx_users_is_active ON users(is_active);

CREATE TABLE sessions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID         NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash  VARCHAR(255) NOT NULL UNIQUE,
    expires_at  TIMESTAMPTZ  NOT NULL,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sessions_user_id    ON sessions(user_id);
CREATE INDEX idx_sessions_token_hash ON sessions(token_hash);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);

CREATE TABLE audit_log (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID         REFERENCES users(id) ON DELETE SET NULL,
    action      VARCHAR(100) NOT NULL,
    resource    VARCHAR(100) NOT NULL,
    resource_id VARCHAR(255),
    ip_address  INET,
    details     JSONB,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_log_user_id    ON audit_log(user_id);
CREATE INDEX idx_audit_log_resource   ON audit_log(resource);
CREATE INDEX idx_audit_log_created_at ON audit_log(created_at DESC);
CREATE INDEX idx_audit_log_action     ON audit_log(action);

-- =============================================================================
-- FINANCE: CHART OF ACCOUNTS & JOURNAL
-- =============================================================================

CREATE TABLE chart_of_accounts (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code         VARCHAR(20)  NOT NULL UNIQUE,
    name         VARCHAR(255) NOT NULL,
    account_type account_type NOT NULL,
    parent_id    UUID         REFERENCES chart_of_accounts(id) ON DELETE RESTRICT,
    is_active    BOOLEAN      NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_coa_code         ON chart_of_accounts(code);
CREATE INDEX idx_coa_account_type ON chart_of_accounts(account_type);
CREATE INDEX idx_coa_parent_id    ON chart_of_accounts(parent_id);
CREATE INDEX idx_coa_is_active    ON chart_of_accounts(is_active);

CREATE TABLE journal_entries (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entry_date  DATE         NOT NULL,
    description TEXT         NOT NULL,
    reference   VARCHAR(100),
    created_by  UUID         REFERENCES users(id) ON DELETE SET NULL,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_journal_entries_entry_date  ON journal_entries(entry_date DESC);
CREATE INDEX idx_journal_entries_reference   ON journal_entries(reference);
CREATE INDEX idx_journal_entries_created_by  ON journal_entries(created_by);

CREATE TABLE journal_lines (
    id            UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    entry_id      UUID           NOT NULL REFERENCES journal_entries(id) ON DELETE CASCADE,
    account_id    UUID           NOT NULL REFERENCES chart_of_accounts(id) ON DELETE RESTRICT,
    debit_amount  NUMERIC(18,2)  NOT NULL DEFAULT 0 CHECK (debit_amount  >= 0),
    credit_amount NUMERIC(18,2)  NOT NULL DEFAULT 0 CHECK (credit_amount >= 0),
    description   TEXT,
    CONSTRAINT chk_journal_lines_amount CHECK (
        (debit_amount > 0 AND credit_amount = 0) OR
        (credit_amount > 0 AND debit_amount = 0)
    )
);

CREATE INDEX idx_journal_lines_entry_id   ON journal_lines(entry_id);
CREATE INDEX idx_journal_lines_account_id ON journal_lines(account_id);

-- =============================================================================
-- FINANCE: CURRENCIES
-- =============================================================================

CREATE TABLE currencies (
    id            UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    code          CHAR(3)        NOT NULL UNIQUE,
    name          VARCHAR(100)   NOT NULL,
    symbol        VARCHAR(10)    NOT NULL,
    exchange_rate NUMERIC(18,6)  NOT NULL DEFAULT 1,
    is_base       BOOLEAN        NOT NULL DEFAULT false,
    updated_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_currencies_code    ON currencies(code);
CREATE INDEX idx_currencies_is_base ON currencies(is_base);

-- =============================================================================
-- STOCK: WAREHOUSES & LOCATIONS
-- =============================================================================

CREATE TABLE warehouses (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       VARCHAR(255) NOT NULL,
    code       VARCHAR(50)  NOT NULL UNIQUE,
    address    TEXT,
    is_active  BOOLEAN      NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_warehouses_code      ON warehouses(code);
CREATE INDEX idx_warehouses_is_active ON warehouses(is_active);

CREATE TABLE warehouse_locations (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    warehouse_id UUID         NOT NULL REFERENCES warehouses(id) ON DELETE CASCADE,
    code         VARCHAR(50)  NOT NULL,
    name         VARCHAR(255),
    row          VARCHAR(20),
    shelf        VARCHAR(20),
    bin          VARCHAR(20),
    UNIQUE (warehouse_id, code)
);

CREATE INDEX idx_warehouse_locations_warehouse_id ON warehouse_locations(warehouse_id);

-- =============================================================================
-- STOCK: CATEGORIES & PRODUCTS
-- =============================================================================

CREATE TABLE product_categories (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        VARCHAR(255) NOT NULL,
    parent_id   UUID         REFERENCES product_categories(id) ON DELETE RESTRICT,
    description TEXT,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_categories_parent_id ON product_categories(parent_id);

-- CRM current_accounts is referenced by products (supplier_id), define it early
CREATE TABLE current_accounts (
    id           UUID                 PRIMARY KEY DEFAULT gen_random_uuid(),
    code         VARCHAR(50)          NOT NULL UNIQUE,
    account_type current_account_type NOT NULL,
    company_name VARCHAR(255)         NOT NULL,
    contact_name VARCHAR(255),
    tax_no       VARCHAR(50),
    tax_office   VARCHAR(100),
    phone        VARCHAR(50),
    email        VARCHAR(255),
    address      TEXT,
    city         VARCHAR(100),
    country      VARCHAR(100)  DEFAULT 'Türkiye',
    notes        TEXT,
    credit_limit NUMERIC(18,2) DEFAULT 0,
    is_active    BOOLEAN       NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_current_accounts_code         ON current_accounts(code);
CREATE INDEX idx_current_accounts_account_type ON current_accounts(account_type);
CREATE INDEX idx_current_accounts_company_name ON current_accounts(company_name);
CREATE INDEX idx_current_accounts_tax_no       ON current_accounts(tax_no);
CREATE INDEX idx_current_accounts_is_active    ON current_accounts(is_active);

CREATE TABLE products (
    id            UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    sku           VARCHAR(100)   NOT NULL UNIQUE,
    name          VARCHAR(255)   NOT NULL,
    description   TEXT,
    category_id   UUID           REFERENCES product_categories(id) ON DELETE SET NULL,
    unit          VARCHAR(20)    NOT NULL DEFAULT 'adet',
    purchase_price NUMERIC(18,4) NOT NULL DEFAULT 0,
    sale_price    NUMERIC(18,4)  NOT NULL DEFAULT 0,
    vat_rate      NUMERIC(5,2)   NOT NULL DEFAULT 18,
    min_stock     NUMERIC(18,3)  NOT NULL DEFAULT 0,
    reorder_point NUMERIC(18,3)  NOT NULL DEFAULT 0,
    barcode       VARCHAR(100),
    is_active     BOOLEAN        NOT NULL DEFAULT true,
    supplier_id   UUID           REFERENCES current_accounts(id) ON DELETE SET NULL,
    created_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_products_sku         ON products(sku);
CREATE INDEX idx_products_name        ON products(name);
CREATE INDEX idx_products_category_id ON products(category_id);
CREATE INDEX idx_products_barcode     ON products(barcode);
CREATE INDEX idx_products_supplier_id ON products(supplier_id);
CREATE INDEX idx_products_is_active   ON products(is_active);

CREATE TABLE product_variants (
    id             UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id     UUID           NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    sku            VARCHAR(100)   NOT NULL UNIQUE,
    name           VARCHAR(255)   NOT NULL,
    attributes     JSONB          NOT NULL DEFAULT '{}',
    price_modifier NUMERIC(18,4)  NOT NULL DEFAULT 0,
    created_at     TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_product_variants_product_id ON product_variants(product_id);
CREATE INDEX idx_product_variants_sku        ON product_variants(sku);
CREATE INDEX idx_product_variants_attributes ON product_variants USING GIN(attributes);

CREATE TABLE serial_numbers (
    id          UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id  UUID          NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    serial_no   VARCHAR(255)  NOT NULL UNIQUE,
    lot_no      VARCHAR(100),
    expiry_date DATE,
    status      serial_status NOT NULL DEFAULT 'available',
    created_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_serial_numbers_product_id ON serial_numbers(product_id);
CREATE INDEX idx_serial_numbers_serial_no  ON serial_numbers(serial_no);
CREATE INDEX idx_serial_numbers_lot_no     ON serial_numbers(lot_no);
CREATE INDEX idx_serial_numbers_status     ON serial_numbers(status);

-- =============================================================================
-- STOCK: MOVEMENTS & LEVELS
-- =============================================================================

CREATE TABLE stock_levels (
    product_id   UUID          NOT NULL REFERENCES products(id)   ON DELETE CASCADE,
    warehouse_id UUID          NOT NULL REFERENCES warehouses(id) ON DELETE CASCADE,
    quantity     NUMERIC(18,3) NOT NULL DEFAULT 0,
    updated_at   TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    PRIMARY KEY (product_id, warehouse_id)
);

CREATE INDEX idx_stock_levels_product_id   ON stock_levels(product_id);
CREATE INDEX idx_stock_levels_warehouse_id ON stock_levels(warehouse_id);

CREATE TABLE stock_movements (
    id             UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id     UUID          NOT NULL REFERENCES products(id)   ON DELETE RESTRICT,
    warehouse_id   UUID          NOT NULL REFERENCES warehouses(id) ON DELETE RESTRICT,
    movement_type  movement_type NOT NULL,
    quantity       NUMERIC(18,3) NOT NULL CHECK (quantity != 0),
    unit_price     NUMERIC(18,4) NOT NULL DEFAULT 0,
    reference_type VARCHAR(50),
    reference_id   UUID,
    notes          TEXT,
    created_by     UUID          REFERENCES users(id) ON DELETE SET NULL,
    created_at     TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_stock_movements_product_id     ON stock_movements(product_id);
CREATE INDEX idx_stock_movements_warehouse_id   ON stock_movements(warehouse_id);
CREATE INDEX idx_stock_movements_movement_type  ON stock_movements(movement_type);
CREATE INDEX idx_stock_movements_reference      ON stock_movements(reference_type, reference_id);
CREATE INDEX idx_stock_movements_created_at     ON stock_movements(created_at DESC);

-- Trigger: keep stock_levels in sync with stock_movements
CREATE OR REPLACE FUNCTION update_stock_level()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO stock_levels (product_id, warehouse_id, quantity, updated_at)
    VALUES (NEW.product_id, NEW.warehouse_id, NEW.quantity, NOW())
    ON CONFLICT (product_id, warehouse_id) DO UPDATE
    SET quantity   = stock_levels.quantity + NEW.quantity,
        updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_stock_movement_update_level
AFTER INSERT ON stock_movements
FOR EACH ROW EXECUTE FUNCTION update_stock_level();

-- =============================================================================
-- HR: DEPARTMENTS, POSITIONS, EMPLOYEES
-- =============================================================================

CREATE TABLE departments (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       VARCHAR(255) NOT NULL,
    parent_id  UUID         REFERENCES departments(id) ON DELETE SET NULL,
    manager_id UUID,  -- FK to employees added after employees table
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_departments_parent_id  ON departments(parent_id);
CREATE INDEX idx_departments_manager_id ON departments(manager_id);

CREATE TABLE positions (
    id            UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    title         VARCHAR(255)   NOT NULL,
    department_id UUID           REFERENCES departments(id) ON DELETE SET NULL,
    min_salary    NUMERIC(18,2)  NOT NULL DEFAULT 0,
    max_salary    NUMERIC(18,2)  NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_positions_department_id ON positions(department_id);

CREATE TABLE employees (
    id                  UUID            PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_no         VARCHAR(50)     NOT NULL UNIQUE,
    first_name          VARCHAR(100)    NOT NULL,
    last_name           VARCHAR(100)    NOT NULL,
    national_id         VARCHAR(20)     UNIQUE,
    birth_date          DATE,
    hire_date           DATE            NOT NULL,
    termination_date    DATE,
    department_id       UUID            REFERENCES departments(id) ON DELETE SET NULL,
    position_id         UUID            REFERENCES positions(id)   ON DELETE SET NULL,
    employment_type     employment_type NOT NULL DEFAULT 'full_time',
    base_salary         NUMERIC(18,2)   NOT NULL DEFAULT 0,
    bank_iban           VARCHAR(50),
    tax_id              VARCHAR(20),
    insurance_no        VARCHAR(50),
    user_id             UUID            UNIQUE REFERENCES users(id) ON DELETE SET NULL,
    emergency_contact   JSONB           NOT NULL DEFAULT '{}',
    documents           JSONB           NOT NULL DEFAULT '[]',
    is_active           BOOLEAN         NOT NULL DEFAULT true,
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_employees_employee_no    ON employees(employee_no);
CREATE INDEX idx_employees_national_id    ON employees(national_id);
CREATE INDEX idx_employees_department_id  ON employees(department_id);
CREATE INDEX idx_employees_position_id    ON employees(position_id);
CREATE INDEX idx_employees_user_id        ON employees(user_id);
CREATE INDEX idx_employees_is_active      ON employees(is_active);
CREATE INDEX idx_employees_hire_date      ON employees(hire_date);

-- Now add the FK from departments.manager_id to employees
ALTER TABLE departments
    ADD CONSTRAINT fk_departments_manager
    FOREIGN KEY (manager_id) REFERENCES employees(id) ON DELETE SET NULL;

CREATE TABLE leave_requests (
    id          UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID         NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    leave_type  leave_type   NOT NULL,
    start_date  DATE         NOT NULL,
    end_date    DATE         NOT NULL,
    days        NUMERIC(5,1) NOT NULL,
    reason      TEXT,
    status      leave_status NOT NULL DEFAULT 'pending',
    approved_by UUID         REFERENCES employees(id) ON DELETE SET NULL,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_leave_dates CHECK (end_date >= start_date)
);

CREATE INDEX idx_leave_requests_employee_id ON leave_requests(employee_id);
CREATE INDEX idx_leave_requests_status      ON leave_requests(status);
CREATE INDEX idx_leave_requests_start_date  ON leave_requests(start_date);

CREATE TABLE attendance (
    id             UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id    UUID          NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    date           DATE          NOT NULL,
    check_in       TIME,
    check_out      TIME,
    overtime_hours NUMERIC(4,2)  NOT NULL DEFAULT 0,
    notes          TEXT,
    UNIQUE (employee_id, date)
);

CREATE INDEX idx_attendance_employee_id ON attendance(employee_id);
CREATE INDEX idx_attendance_date        ON attendance(date DESC);

CREATE TABLE payroll (
    id                   UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id          UUID           NOT NULL REFERENCES employees(id) ON DELETE RESTRICT,
    period_year          SMALLINT       NOT NULL,
    period_month         SMALLINT       NOT NULL CHECK (period_month BETWEEN 1 AND 12),
    gross_salary         NUMERIC(18,2)  NOT NULL DEFAULT 0,
    overtime_pay         NUMERIC(18,2)  NOT NULL DEFAULT 0,
    bonuses              NUMERIC(18,2)  NOT NULL DEFAULT 0,
    deductions           NUMERIC(18,2)  NOT NULL DEFAULT 0,
    ssk_employee         NUMERIC(18,2)  NOT NULL DEFAULT 0,
    ssk_employer         NUMERIC(18,2)  NOT NULL DEFAULT 0,
    income_tax           NUMERIC(18,2)  NOT NULL DEFAULT 0,
    stamp_tax            NUMERIC(18,2)  NOT NULL DEFAULT 0,
    unemployment_insurance NUMERIC(18,2) NOT NULL DEFAULT 0,
    net_salary           NUMERIC(18,2)  NOT NULL DEFAULT 0,
    payment_date         DATE,
    status               payroll_status NOT NULL DEFAULT 'draft',
    created_at           TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    UNIQUE (employee_id, period_year, period_month)
);

CREATE INDEX idx_payroll_employee_id ON payroll(employee_id);
CREATE INDEX idx_payroll_period      ON payroll(period_year DESC, period_month DESC);
CREATE INDEX idx_payroll_status      ON payroll(status);

-- =============================================================================
-- CRM: CONTACTS & TASKS
-- =============================================================================

CREATE TABLE contacts (
    id          UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id  UUID         NOT NULL REFERENCES current_accounts(id) ON DELETE CASCADE,
    first_name  VARCHAR(100) NOT NULL,
    last_name   VARCHAR(100),
    title       VARCHAR(100),
    phone       VARCHAR(50),
    email       VARCHAR(255),
    is_primary  BOOLEAN      NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_contacts_account_id ON contacts(account_id);
CREATE INDEX idx_contacts_email      ON contacts(email);

CREATE TABLE tasks (
    id          UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id  UUID          REFERENCES current_accounts(id) ON DELETE SET NULL,
    assigned_to UUID          REFERENCES users(id) ON DELETE SET NULL,
    title       VARCHAR(255)  NOT NULL,
    description TEXT,
    due_date    DATE,
    priority    task_priority NOT NULL DEFAULT 'medium',
    status      task_status   NOT NULL DEFAULT 'open',
    created_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tasks_account_id  ON tasks(account_id);
CREATE INDEX idx_tasks_assigned_to ON tasks(assigned_to);
CREATE INDEX idx_tasks_status      ON tasks(status);
CREATE INDEX idx_tasks_due_date    ON tasks(due_date);
CREATE INDEX idx_tasks_priority    ON tasks(priority);

-- =============================================================================
-- BANK: ACCOUNTS & TRANSACTIONS
-- =============================================================================

CREATE TABLE bank_accounts (
    id              UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    account_name    VARCHAR(255)   NOT NULL,
    bank_name       VARCHAR(255)   NOT NULL,
    iban            VARCHAR(50)    NOT NULL UNIQUE,
    account_no      VARCHAR(50),
    currency_id     UUID           NOT NULL REFERENCES currencies(id) ON DELETE RESTRICT,
    current_balance NUMERIC(18,2)  NOT NULL DEFAULT 0,
    is_active       BOOLEAN        NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bank_accounts_iban        ON bank_accounts(iban);
CREATE INDEX idx_bank_accounts_currency_id ON bank_accounts(currency_id);
CREATE INDEX idx_bank_accounts_is_active   ON bank_accounts(is_active);

CREATE TABLE bank_transactions (
    id              UUID                  PRIMARY KEY DEFAULT gen_random_uuid(),
    bank_account_id UUID                  NOT NULL REFERENCES bank_accounts(id) ON DELETE RESTRICT,
    transaction_date DATE                 NOT NULL,
    description     TEXT                  NOT NULL,
    amount          NUMERIC(18,2)         NOT NULL,
    transaction_type transaction_type_bank NOT NULL,
    reference       VARCHAR(100),
    balance_after   NUMERIC(18,2)         NOT NULL,
    created_by      UUID                  REFERENCES users(id) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ           NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bank_transactions_bank_account_id   ON bank_transactions(bank_account_id);
CREATE INDEX idx_bank_transactions_transaction_date  ON bank_transactions(transaction_date DESC);
CREATE INDEX idx_bank_transactions_transaction_type  ON bank_transactions(transaction_type);

-- =============================================================================
-- CASH: REGISTERS & TRANSACTIONS
-- =============================================================================

CREATE TABLE cash_registers (
    id              UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(255)  NOT NULL,
    code            VARCHAR(50)   NOT NULL UNIQUE,
    currency_id     UUID          NOT NULL REFERENCES currencies(id) ON DELETE RESTRICT,
    current_balance NUMERIC(18,2) NOT NULL DEFAULT 0,
    is_active       BOOLEAN       NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cash_registers_code        ON cash_registers(code);
CREATE INDEX idx_cash_registers_currency_id ON cash_registers(currency_id);

CREATE TABLE cash_transactions (
    id               UUID                   PRIMARY KEY DEFAULT gen_random_uuid(),
    register_id      UUID                   NOT NULL REFERENCES cash_registers(id) ON DELETE RESTRICT,
    transaction_date DATE                   NOT NULL,
    description      TEXT                   NOT NULL,
    amount           NUMERIC(18,2)          NOT NULL CHECK (amount > 0),
    transaction_type transaction_type_cash  NOT NULL,
    reference        VARCHAR(100),
    balance_after    NUMERIC(18,2)          NOT NULL,
    created_by       UUID                   REFERENCES users(id) ON DELETE SET NULL,
    created_at       TIMESTAMPTZ            NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cash_transactions_register_id      ON cash_transactions(register_id);
CREATE INDEX idx_cash_transactions_transaction_date ON cash_transactions(transaction_date DESC);

-- =============================================================================
-- INVOICE: INVOICES, LINES, SHIPMENTS
-- =============================================================================

CREATE TABLE invoices (
    id            UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_no    VARCHAR(50)    NOT NULL UNIQUE,
    invoice_type  invoice_type   NOT NULL,
    account_id    UUID           NOT NULL REFERENCES current_accounts(id) ON DELETE RESTRICT,
    invoice_date  DATE           NOT NULL,
    due_date      DATE,
    currency_id   UUID           NOT NULL REFERENCES currencies(id) ON DELETE RESTRICT,
    exchange_rate NUMERIC(18,6)  NOT NULL DEFAULT 1,
    subtotal      NUMERIC(18,2)  NOT NULL DEFAULT 0,
    vat_amount    NUMERIC(18,2)  NOT NULL DEFAULT 0,
    total_amount  NUMERIC(18,2)  NOT NULL DEFAULT 0,
    paid_amount   NUMERIC(18,2)  NOT NULL DEFAULT 0,
    status        invoice_status NOT NULL DEFAULT 'draft',
    notes         TEXT,
    created_by    UUID           REFERENCES users(id) ON DELETE SET NULL,
    created_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_invoices_invoice_no   ON invoices(invoice_no);
CREATE INDEX idx_invoices_invoice_type ON invoices(invoice_type);
CREATE INDEX idx_invoices_account_id   ON invoices(account_id);
CREATE INDEX idx_invoices_invoice_date ON invoices(invoice_date DESC);
CREATE INDEX idx_invoices_due_date     ON invoices(due_date);
CREATE INDEX idx_invoices_status       ON invoices(status);
CREATE INDEX idx_invoices_created_by   ON invoices(created_by);

CREATE TABLE invoice_lines (
    id          UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_id  UUID          NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    product_id  UUID          REFERENCES products(id) ON DELETE SET NULL,
    description TEXT          NOT NULL,
    quantity    NUMERIC(18,3) NOT NULL CHECK (quantity > 0),
    unit_price  NUMERIC(18,4) NOT NULL DEFAULT 0,
    vat_rate    NUMERIC(5,2)  NOT NULL DEFAULT 0,
    vat_amount  NUMERIC(18,2) NOT NULL DEFAULT 0,
    line_total  NUMERIC(18,2) NOT NULL DEFAULT 0
);

CREATE INDEX idx_invoice_lines_invoice_id ON invoice_lines(invoice_id);
CREATE INDEX idx_invoice_lines_product_id ON invoice_lines(product_id);

CREATE TABLE shipments (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_id    UUID        NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    shipment_date DATE        NOT NULL,
    tracking_no   VARCHAR(100),
    notes         TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_shipments_invoice_id ON shipments(invoice_id);

-- =============================================================================
-- PAYMENT PLANS
-- =============================================================================

CREATE TABLE payment_plans (
    id         UUID                PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_id UUID                NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    due_date   DATE                NOT NULL,
    amount     NUMERIC(18,2)       NOT NULL CHECK (amount > 0),
    status     payment_plan_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ         NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payment_plans_invoice_id ON payment_plans(invoice_id);
CREATE INDEX idx_payment_plans_due_date   ON payment_plans(due_date);
CREATE INDEX idx_payment_plans_status     ON payment_plans(status);

-- =============================================================================
-- CHECKS
-- =============================================================================

CREATE TABLE checks (
    id          UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    check_no    VARCHAR(100)  NOT NULL,
    account_id  UUID          NOT NULL REFERENCES current_accounts(id) ON DELETE RESTRICT,
    bank_name   VARCHAR(255),
    amount      NUMERIC(18,2) NOT NULL CHECK (amount > 0),
    issue_date  DATE          NOT NULL,
    due_date    DATE          NOT NULL,
    check_type  check_type    NOT NULL,
    status      check_status  NOT NULL DEFAULT 'pending',
    notes       TEXT,
    created_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_check_dates CHECK (due_date >= issue_date)
);

CREATE INDEX idx_checks_check_no   ON checks(check_no);
CREATE INDEX idx_checks_account_id ON checks(account_id);
CREATE INDEX idx_checks_due_date   ON checks(due_date);
CREATE INDEX idx_checks_status     ON checks(status);
CREATE INDEX idx_checks_check_type ON checks(check_type);

-- =============================================================================
-- SEED DATA
-- =============================================================================

-- Base currencies
INSERT INTO currencies (id, code, name, symbol, exchange_rate, is_base) VALUES
    (gen_random_uuid(), 'TRY', 'Türk Lirası',     '₺', 1.000000, true),
    (gen_random_uuid(), 'USD', 'US Dollar',         '$', 0.031000, false),
    (gen_random_uuid(), 'EUR', 'Euro',              '€', 0.029000, false),
    (gen_random_uuid(), 'GBP', 'British Pound',     '£', 0.025000, false)
ON CONFLICT (code) DO NOTHING;

-- Default chart of accounts (Turkish TEKDÜZEN accounting plan - abbreviated)
INSERT INTO chart_of_accounts (id, code, name, account_type, parent_id, is_active) VALUES
    (gen_random_uuid(), '100', 'Kasa',                          'asset',     NULL, true),
    (gen_random_uuid(), '102', 'Bankalar',                      'asset',     NULL, true),
    (gen_random_uuid(), '120', 'Alıcılar',                      'asset',     NULL, true),
    (gen_random_uuid(), '153', 'Ticari Mallar',                 'asset',     NULL, true),
    (gen_random_uuid(), '191', 'İndirilecek KDV',               'asset',     NULL, true),
    (gen_random_uuid(), '200', 'Arazi ve Arsalar',              'asset',     NULL, true),
    (gen_random_uuid(), '253', 'Tesis, Makina ve Cihazlar',     'asset',     NULL, true),
    (gen_random_uuid(), '320', 'Satıcılar',                     'liability', NULL, true),
    (gen_random_uuid(), '360', 'Ödenecek Vergiler ve Fonlar',   'liability', NULL, true),
    (gen_random_uuid(), '361', 'Ödenecek Sosyal Güvenlik Kes.', 'liability', NULL, true),
    (gen_random_uuid(), '391', 'Hesaplanan KDV',                'liability', NULL, true),
    (gen_random_uuid(), '500', 'Sermaye',                       'equity',    NULL, true),
    (gen_random_uuid(), '570', 'Geçmiş Yıllar Kârları',         'equity',    NULL, true),
    (gen_random_uuid(), '600', 'Yurt İçi Satışlar',             'revenue',   NULL, true),
    (gen_random_uuid(), '601', 'Yurt Dışı Satışlar',            'revenue',   NULL, true),
    (gen_random_uuid(), '610', 'Satıştan İadeler',              'revenue',   NULL, true),
    (gen_random_uuid(), '620', 'Satılan Ticari Mallar Maliyeti','expense',   NULL, true),
    (gen_random_uuid(), '630', 'Araştırma ve Geliştirme Gid.',  'expense',   NULL, true),
    (gen_random_uuid(), '631', 'Pazarlama Satış Dağıtım Gid.',  'expense',   NULL, true),
    (gen_random_uuid(), '632', 'Genel Yönetim Giderleri',       'expense',   NULL, true),
    (gen_random_uuid(), '660', 'Kısa Vadeli Borçlanma Gid.',    'expense',   NULL, true),
    (gen_random_uuid(), '770', 'Genel Yönetim Giderleri',       'expense',   NULL, true),
    (gen_random_uuid(), '780', 'Finansman Giderleri',           'expense',   NULL, true)
ON CONFLICT (code) DO NOTHING;

-- Default warehouse
INSERT INTO warehouses (id, name, code, address, is_active) VALUES
    (gen_random_uuid(), 'Ana Depo', 'WH001', 'Merkez Depo, Türkiye', true)
ON CONFLICT (code) DO NOTHING;

-- Default admin user (password: admin, salt: admin_salt)
INSERT INTO users (id, username, email, password_hash, salt, role, is_active)
VALUES (
    gen_random_uuid(),
    'admin',
    'admin@erp.local',
    encode(sha256(('admin_salt' || 'admin')::bytea), 'hex'),
    'admin_salt',
    'admin',
    true
) ON CONFLICT (username) DO NOTHING;
