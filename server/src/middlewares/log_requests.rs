use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use tokio::time::Instant;

pub async fn log_requests(request: Request<Body>, next: Next) -> Response<Body> {
    let start_time = Instant::now();

    let method = request.method().clone();
    let uri = request.uri().clone();
    let version = request.version();

    // Pass the request to the next middleware/handler
    let response = next.run(request).await;

    let duration = start_time.elapsed();
    let status = response.status().as_u16();

    let log_message = format!(
        "[{} {} {:?}] - {} - {:?}",
        method, uri, version, status, duration
    );

    if response.status().is_client_error() || response.status().is_server_error() {
        log::error!("{}", log_message);
    } else {
        log::info!("{}", log_message);
    }

    response
}
