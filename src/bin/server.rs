use container_registry_gateway::{configuration, server, shutdown};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> container_registry_gateway::Result<()> {
    set_up_logging()?;

    let configuration = configuration::load(&[])?;

    let tcp_listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.http_server.host, configuration.http_server.port
    ))
    .await?;

    server::run(tcp_listener.into_std()?, shutdown::recv(), configuration).await?;

    Ok(())
}

fn set_up_logging() -> container_registry_gateway::Result<()> {
    tracing_subscriber::fmt::try_init()
}
