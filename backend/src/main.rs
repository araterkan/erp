mod config;
mod db;
mod error;
mod middleware;
mod domain;

use axum::{
    middleware as axum_middleware,
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use config::Config;
use db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "erp_backend=debug,tower_http=debug".into()),
        )
        .init();

    tracing::info!("Starting ERP Backend...");

    // Load config
    let config = Config::from_env()?;
    let server_addr = format!("{}:{}", config.server_host, config.server_port);

    // Create DB pool
    let pool = db::create_pool(&config)?;

    // Run migrations
    tracing::info!("Running database migrations...");
    if let Err(e) = db::run_migrations(&pool).await {
        tracing::warn!("Migration warning (may be normal on first run): {}", e);
    }

    // Seed admin user
    if let Err(e) = db::seed_admin_user(&pool).await {
        tracing::warn!("Seed warning: {}", e);
    }

    let state = AppState {
        pool,
        config: Arc::new(config),
    };

    // CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application routes
    let app = Router::new()
        .merge(public_routes(state.clone()))
        .merge(protected_routes(state.clone()))
        .layer(cors)
        .layer(axum_middleware::from_fn(middleware::logger::request_logger));

    tracing::info!("Server listening on {}", server_addr);
    let listener = tokio::net::TcpListener::bind(&server_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn public_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/auth/login", post(domain::auth::handlers::login))
        .route("/health", get(health_check))
        .with_state(state)
}

fn protected_routes(state: AppState) -> Router {
    let auth_routes = Router::new()
        .route("/logout", post(domain::auth::handlers::logout))
        .route("/me", get(domain::auth::handlers::me))
        .route("/change-password", post(domain::auth::handlers::change_password))
        .route("/users", get(domain::auth::handlers::list_users))
        .route("/users", post(domain::auth::handlers::create_user))
        .route("/users/{id}", put(domain::auth::handlers::update_user))
        .route("/users/{id}", delete(domain::auth::handlers::delete_user));

    let finance_routes = Router::new()
        .route("/accounts", get(domain::finance::handlers::list_accounts))
        .route("/accounts", post(domain::finance::handlers::create_account))
        .route("/accounts/{id}", get(domain::finance::handlers::get_account))
        .route("/journal-entries", get(domain::finance::handlers::list_journal_entries))
        .route("/journal-entries", post(domain::finance::handlers::create_journal_entry))
        .route("/trial-balance", get(domain::finance::handlers::get_trial_balance));

    let stock_routes = Router::new()
        .route("/warehouses", get(domain::stock::handlers::list_warehouses))
        .route("/warehouses", post(domain::stock::handlers::create_warehouse))
        .route("/products", get(domain::stock::handlers::list_products))
        .route("/products", post(domain::stock::handlers::create_product))
        .route("/products/{id}", get(domain::stock::handlers::get_product))
        .route("/movements", post(domain::stock::handlers::create_stock_movement))
        .route("/levels", get(domain::stock::handlers::get_stock_levels));

    let hr_routes = Router::new()
        .route("/departments", get(domain::hr::handlers::list_departments))
        .route("/departments", post(domain::hr::handlers::create_department))
        .route("/employees", get(domain::hr::handlers::list_employees))
        .route("/employees", post(domain::hr::handlers::create_employee))
        .route("/employees/{id}", get(domain::hr::handlers::get_employee))
        .route("/leave-requests", get(domain::hr::handlers::list_leave_requests))
        .route("/leave-requests", post(domain::hr::handlers::create_leave_request))
        .route("/payroll-runs", get(domain::hr::handlers::list_payroll_runs))
        .route("/payroll-runs", post(domain::hr::handlers::create_payroll_run));

    let crm_routes = Router::new()
        .route("/customers", get(domain::crm::handlers::list_customers))
        .route("/customers", post(domain::crm::handlers::create_customer))
        .route("/customers/{id}", get(domain::crm::handlers::get_customer))
        .route("/contacts", get(domain::crm::handlers::list_contacts))
        .route("/contacts", post(domain::crm::handlers::create_contact));

    let bank_routes = Router::new()
        .route("/accounts", get(domain::bank::handlers::list_accounts))
        .route("/accounts", post(domain::bank::handlers::create_account))
        .route("/accounts/{id}/transactions", get(domain::bank::handlers::list_transactions))
        .route("/transactions", post(domain::bank::handlers::create_transaction));

    let cash_routes = Router::new()
        .route("/registers", get(domain::cash::handlers::list_registers))
        .route("/registers", post(domain::cash::handlers::create_register))
        .route("/registers/{id}/transactions", get(domain::cash::handlers::list_transactions))
        .route("/transactions", post(domain::cash::handlers::create_transaction));

    let invoice_routes = Router::new()
        .route("/", get(domain::invoice::handlers::list_invoices))
        .route("/", post(domain::invoice::handlers::create_invoice))
        .route("/{id}", get(domain::invoice::handlers::get_invoice));

    let report_routes = Router::new()
        .route("/dashboard", get(domain::report::handlers::dashboard))
        .route("/sales", get(domain::report::handlers::sales_report))
        .route("/inventory", get(domain::report::handlers::inventory_report));

    Router::new()
        .nest("/api/auth", auth_routes)
        .nest("/api/finance", finance_routes)
        .nest("/api/stock", stock_routes)
        .nest("/api/hr", hr_routes)
        .nest("/api/crm", crm_routes)
        .nest("/api/bank", bank_routes)
        .nest("/api/cash", cash_routes)
        .nest("/api/invoices", invoice_routes)
        .nest("/api/reports", report_routes)
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::audit::audit_trail,
        ))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::require_auth,
        ))
        .with_state(state)
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "service": "ERP Backend",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
