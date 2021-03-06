use std::{future::Future, net::TcpListener};

use axum::{
    routing::{any, get},
    Extension, Router, Server,
};

use crate::{configuration, http, oci, route, snyk, state};

/// # Errors
///
/// Returns `Err` if the server fails to start.
pub async fn run(
    tcp_listener: TcpListener,
    shutdown_signal: impl Future<Output = ()>,
    configuration: configuration::Configuration,
) -> crate::Result<()> {
    let socket_addr = tcp_listener.local_addr()?;

    let state = state::State {
        http_client: http::client(),
        oci_proxy: oci::Proxy::new(configuration.oci.base_address),
        oci_regex: oci::Regex::default(),
        snyk_api: snyk::Api::new(
            configuration.snyk.base_address,
            configuration.snyk.api_key,
            configuration.snyk.organization_id,
            configuration.snyk.integration_id,
        ),
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
