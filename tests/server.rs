use container_registry_gateway::{
    configuration,
    oci::{Response, ResponseError},
    server, shutdown,
};
use hyper::{body::Buf as _, client::Client, StatusCode};
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

    assert_eq!(StatusCode::NOT_FOUND, response.status());
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

    assert_eq!(StatusCode::OK, response.status());
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

    assert_eq!(StatusCode::OK, response.status());
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

    assert_eq!(StatusCode::UNAUTHORIZED, response.status());

    let body = parse_body(response).await;

    assert_eq!(
        Response {
            errors: vec![ResponseError {
                code: "UNAUTHORIZED".to_string(),
                message: "authentication required".to_string(),
                details: None
            }]
        },
        body
    );
}

async fn start_server() -> SocketAddr {
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();

    let configuration = configuration::load(&[
        ("oci.base_address", "https://registry-1.docker.io"),
        ("snyk.api_key", ""),
        ("snyk.base_address", ""),
        ("snyk.integration_id", ""),
        ("snyk.organization_id", ""),
    ])
    .unwrap();

    let socket_addr = tcp_listener.local_addr().unwrap();

    tokio::spawn(async move {
        server::run(
            tcp_listener.into_std().unwrap(),
            shutdown::recv(),
            configuration,
        )
        .await
    });

    socket_addr
}

async fn parse_body(response: hyper::Response<hyper::Body>) -> Response {
    let buffer = hyper::body::aggregate(response).await.unwrap();

    serde_json::from_reader(buffer.reader()).unwrap()
}
