#[derive(Clone)]
pub(crate) struct State {
    pub(crate) http_client: crate::http::Client,
    pub(crate) oci_proxy: crate::oci::Proxy,
}
