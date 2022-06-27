use container_registry_gateway::{server, signal};
use hyper::client::Client;
use std::net::SocketAddr;

#[tokio::test]
async fn returns_404() {
    let socket_addr = start_server().await;

    let client = Client::new();

    let response = client
        .get(
            format!("http://127.0.0.1:{}", socket_addr.port())
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(404, response.status())
}

async fn start_server() -> SocketAddr {
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();

    let socket_addr = tcp_listener.local_addr().unwrap();

    tokio::spawn(
        async move { server::run(tcp_listener.into_std().unwrap(), signal::shutdown()).await },
    );

    socket_addr
}
