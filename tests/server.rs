use container_registry_gateway::{server, shutdown};
use hyper::{client::Client, StatusCode};
use std::net::SocketAddr;

#[tokio::test]
async fn root_returns_not_found() {
    let socket_addr = start_server().await;

    let response = Client::new()
        .get(
            format!("http://127.0.0.1:{}", socket_addr.port())
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::NOT_FOUND, response.status())
}

#[tokio::test]
async fn health_liveness_get_returns_ok() {
    let socket_addr = start_server().await;

    let response = Client::new()
        .get(
            format!("http://127.0.0.1:{}/health/liveness", socket_addr.port())
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status())
}

#[tokio::test]
async fn health_readiness_get_returns_ok() {
    let socket_addr = start_server().await;

    let response = Client::new()
        .get(
            format!("http://127.0.0.1:{}/health/readiness", socket_addr.port())
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status())
}

#[tokio::test]
async fn v2_root_returns_unauthorized() {
    let socket_addr = start_server().await;

    let response = Client::new()
        .get(
            format!("http://127.0.0.1:{}/v2/", socket_addr.port())
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(StatusCode::UNAUTHORIZED, response.status())
}

async fn start_server() -> SocketAddr {
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();

    let socket_addr = tcp_listener.local_addr().unwrap();

    tokio::spawn(
        async move { server::run(tcp_listener.into_std().unwrap(), shutdown::recv()).await },
    );

    socket_addr
}
