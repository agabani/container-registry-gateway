use std::{future::Future, net::TcpListener};

use axum::{
    routing::{any, get},
    Extension, Router, Server,
};

use crate::{
    http, oci,
    route::{health_liveness_get, health_readiness_get, v2_any},
    state::State,
};

/// # Errors
///
/// Returns `Err` if the server fails to start.
pub async fn run(
    tcp_listener: TcpListener,
    shutdown_signal: impl Future<Output = ()>,
) -> crate::Result<()> {
    let socket_addr = tcp_listener.local_addr()?;

    let state = State {
        http_client: http::client(),
        oci_proxy: oci::Proxy::new("https://registry-1.docker.io"),
    };

    let app = Router::new()
        .route("/health/liveness", get(health_liveness_get))
        .route("/health/readiness", get(health_readiness_get))
        .route("/v2/*path", any(v2_any))
        .layer(Extension(state));

    let server = Server::from_tcp(tcp_listener)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal);

    tracing::info!(%socket_addr, "Server started");

    server.await?;

    tracing::info!("Server stopped");

    Ok(())
}
