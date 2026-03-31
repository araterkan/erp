use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;

use crate::domain::auth::models::Claims;
use crate::AppState;

pub async fn audit_trail(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .or_else(|| req.headers().get("x-real-ip"))
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let claims = req.extensions().get::<Claims>().cloned();

    let response = next.run(req).await;

    let elapsed = start.elapsed().as_millis() as i64;
    let status = response.status().as_u16() as i32;

    if let Ok(client) = state.pool.get().await {
        let user_id = claims.as_ref().map(|c| c.user_id);
        let username = claims.as_ref().map(|c| c.username.clone());
        let action = format!("{} {}", method, path);

        let resource = path.split('/').nth(2).unwrap_or("unknown").to_string();
        let details = serde_json::json!({
            "method": method,
            "status": status,
            "response_time_ms": elapsed,
            "user_agent": user_agent,
            "username": username,
        });
        let _ = client.execute(
            "INSERT INTO audit_log (user_id, action, resource, ip_address, details)
             VALUES ($1, $2, $3, $4::inet, $5)",
            &[
                &user_id,
                &action,
                &resource,
                &ip,
                &details,
            ],
        ).await;
    }

    response
}
