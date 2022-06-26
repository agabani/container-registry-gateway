use std::net::TcpListener;

use axum::{Router, Server};

/// # Errors
///
/// Returns `Err` if the server fails to start.
pub async fn run(tcp_listener: TcpListener) -> crate::Result<()> {
    let app = Router::new();

    let server = Server::from_tcp(tcp_listener)?.serve(app.into_make_service());

    server.await?;

    Ok(())
}
