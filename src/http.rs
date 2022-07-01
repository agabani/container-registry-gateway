pub(crate) type Client = hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>;

/// Creates a new `Client` instance.
pub(crate) fn client() -> Client {
    hyper::Client::builder().build(
        hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .build(),
    )
}
