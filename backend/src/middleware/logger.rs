use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;

pub async fn request_logger(req: Request<Body>, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();

    tracing::info!("→ {} {}", method, uri);

    let response = next.run(req).await;

    let elapsed = start.elapsed();
    let status = response.status();

    tracing::info!(
        "← {} {} {} ({:.2}ms)",
        method,
        uri,
        status.as_u16(),
        elapsed.as_secs_f64() * 1000.0
    );

    response
}
