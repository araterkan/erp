use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;

use crate::domain::auth::models::Claims;
use crate::AppState;

pub async fn require_auth(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let token = extract_token(&req);

    match token {
        Some(token) => {
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
                &Validation::default(),
            ) {
                Ok(token_data) => {
                    req.extensions_mut().insert(token_data.claims);
                    Ok(next.run(req).await)
                }
                Err(_) => Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Invalid or expired token"})),
                )),
            }
        }
        None => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Authorization token required"})),
        )),
    }
}

#[allow(dead_code)]
pub async fn require_admin(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let token = extract_token(&req);

    match token {
        Some(token) => {
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
                &Validation::default(),
            ) {
                Ok(token_data) => {
                    if token_data.claims.role != "admin" {
                        return Err((
                            StatusCode::FORBIDDEN,
                            Json(json!({"error": "Admin access required"})),
                        ));
                    }
                    req.extensions_mut().insert(token_data.claims);
                    Ok(next.run(req).await)
                }
                Err(_) => Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Invalid or expired token"})),
                )),
            }
        }
        None => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Authorization token required"})),
        )),
    }
}

fn extract_token(req: &Request<Body>) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

#[allow(dead_code)]
pub fn get_claims(req: &Request<Body>) -> Option<Claims> {
    req.extensions().get::<Claims>().cloned()
}
