use axum::http::status::StatusCode;

use crate::oci::Proxy;

/// GET /health/liveness
///
/// Returns 200 if the server is healthy.
///
/// This endpoint is used by the Kubernetes liveness probe.
#[allow(clippy::unused_async)]
pub(crate) async fn health_liveness_get() -> StatusCode {
    StatusCode::OK
}

/// GET /health/readiness
///
/// Returns 200 if the server is ready.
///
/// This endpoint is used by the Kubernetes readiness probe.
#[allow(clippy::unused_async)]
pub(crate) async fn health_readiness_get() -> StatusCode {
    StatusCode::OK
}

/// ANY /v2/*
///
/// Returns 501 Not Implemented.
///
/// This endpoint is used by the OCI distribution specification proxy.
#[allow(clippy::unused_async)]
pub(crate) async fn v2_any(
    request: axum::http::Request<axum::body::Body>,
) -> Result<hyper::Response<hyper::Body>, StatusCode> {
    Proxy::new("https://registry-1.docker.io")
        .send(request)
        .await
        .map_err(|error| {
            tracing::error!(?error);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
