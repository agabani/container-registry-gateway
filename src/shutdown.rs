/// Receives the shutdown signal, waiting if necessary.
pub async fn recv() {
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

    let signal = tokio::select! {
        _ = control_c => {
            "Ctrl+C"
        }
        _ = sigint => {
            "SIGINT"
        }
        _ = sigterm => {
            "SIGTERM"
        }
    };

    tracing::info!(signal, "Received signal");
}
