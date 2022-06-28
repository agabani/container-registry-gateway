use axum::http::status::StatusCode;

use crate::oci_proxy::OciProxy;

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
pub(crate) async fn v2_any() -> StatusCode {
    let _oci_proxy = OciProxy::new("https://registry-1.docker.io");

    StatusCode::NOT_IMPLEMENTED
}
