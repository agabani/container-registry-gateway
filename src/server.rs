use std::{future::Future, net::TcpListener};

use axum::{Router, Server};

/// # Errors
///
/// Returns `Err` if the server fails to start.
pub async fn run(
    tcp_listener: TcpListener,
    shutdown_signal: impl Future<Output = ()>,
) -> crate::Result<()> {
    let app = Router::new();

    let server = Server::from_tcp(tcp_listener)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal);

    server.await?;

    Ok(())
}
