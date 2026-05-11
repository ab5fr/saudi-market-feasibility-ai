use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tracing::info;

/// Request logging middleware
pub async fn log_requests(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    info!("→ {} {}", method, uri);
    
    let response = next.run(req).await;
    
    info!("← {} {} - {}", method, uri, response.status());
    
    response
}

/// Simple auth middleware placeholder
pub async fn auth_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // TODO: Implement JWT validation
    // For now, allow all requests
    Ok(next.run(req).await)
}
