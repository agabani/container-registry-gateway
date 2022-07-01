use std::{future::Future, net::TcpListener};

use axum::{
    routing::{any, get},
    Extension, Router, Server,
};

use crate::{http, oci, route, state};

/// # Errors
///
/// Returns `Err` if the server fails to start.
pub async fn run(
    tcp_listener: TcpListener,
    shutdown_signal: impl Future<Output = ()>,
) -> crate::Result<()> {
    let socket_addr = tcp_listener.local_addr()?;

    let state = state::State {
        http_client: http::client(),
        oci_proxy: oci::Proxy::new("https://registry-1.docker.io"),
        oci_regex: oci::Regex::default(),
    };

    let app = Router::new()
        .route("/health/liveness", get(route::health_liveness_get))
        .route("/health/readiness", get(route::health_readiness_get))
        .route("/v2/*path", any(route::v2_routes))
        .layer(Extension(state));

    let server = Server::from_tcp(tcp_listener)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal);

    tracing::info!(%socket_addr, "Server started");

    server.await?;

    tracing::info!("Server stopped");

    Ok(())
}
