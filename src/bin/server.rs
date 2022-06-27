use container_registry_gateway::{server, signals};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> container_registry_gateway::Result<()> {
    set_up_logging()?;

    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await?;

    tracing::info!("Listening on {}", tcp_listener.local_addr()?);

    server::run(tcp_listener.into_std()?, signals::shutdown()).await?;

    Ok(())
}

fn set_up_logging() -> container_registry_gateway::Result<()> {
    tracing_subscriber::fmt::try_init()
}
