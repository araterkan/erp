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

        let _ = client.execute(
            "INSERT INTO audit_logs (user_id, username, action, method, path, status_code, ip_address, user_agent, response_time_ms)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            &[
                &user_id,
                &username,
                &action,
                &method,
                &path,
                &status,
                &ip,
                &user_agent,
                &elapsed,
            ],
        ).await;
    }

    response
}
