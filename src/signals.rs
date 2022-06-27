pub async fn shutdown() {
    let control_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let sigint = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
            .expect("Failed to install SIGINT handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let sigint = std::future::pending::<()>();

    #[cfg(unix)]
    let sigterm = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = control_c => {
            tracing::info!("Received Ctrl+C, shutting down");
        }
        _ = sigint => {
            tracing::info!("Received SIGINT, shutting down");
        }
        _ = sigterm => {
            tracing::info!("Received SIGTERM, shutting down");
        }
    }
}
