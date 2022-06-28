use container_registry_gateway::{server, shutdown};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> container_registry_gateway::Result<()> {
    set_up_logging()?;

    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await?;

    server::run(tcp_listener.into_std()?, shutdown::recv()).await?;

    Ok(())
}

fn set_up_logging() -> container_registry_gateway::Result<()> {
    tracing_subscriber::fmt::try_init()
}
