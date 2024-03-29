#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Response {
    pub errors: Vec<ResponseError>,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ResponseError {
    pub code: String,
    pub message: String,
    pub details: Option<()>,
}

#[derive(Clone)]
pub(crate) struct Proxy {
    base_address: String,
}

pub(crate) struct ProxyRequest {
    base_address: String,
    request: hyper::Request<hyper::Body>,
}

pub(crate) struct ProxyResponse {
    base_address: String,
    response: hyper::Response<hyper::Body>,
}

#[derive(Clone)]
pub(crate) struct Regex {
    pub(crate) name_manifest_reference: regex::Regex,
}

impl Proxy {
    /// Creates a new `Proxy` instance.
    pub(crate) fn new(base_address: impl Into<String>) -> Proxy {
        Proxy {
            base_address: base_address.into(),
        }
    }

    /// Creates a new `ProxyRequest` instance.
    pub(crate) fn request(&self, request: impl Into<hyper::Request<hyper::Body>>) -> ProxyRequest {
        ProxyRequest {
            base_address: self.base_address.clone(),
            request: request.into(),
        }
    }

    /// Creates a new `ProxyResponse` instance.
    pub(crate) fn response(
        &self,
        response: impl Into<hyper::Response<hyper::Body>>,
    ) -> ProxyResponse {
        ProxyResponse {
            base_address: self.base_address.clone(),
            response: response.into(),
        }
    }

    /// Sends a request.
    ///
    /// A convenience method for proxying a request to the backend.
    pub(crate) async fn send(
        &self,
        client: &crate::http::Client,
        request: impl Into<hyper::Request<hyper::Body>>,
    ) -> crate::Result<hyper::Response<hyper::Body>> {
        let proxy_request = self.request(request.into());

        let proxy_response = self.response(client.request(proxy_request.try_into()?).await?);

        proxy_response.try_into()
    }
}

impl TryFrom<ProxyRequest> for hyper::Request<hyper::Body> {
    type Error = crate::Error;

    fn try_from(this: ProxyRequest) -> Result<Self, Self::Error> {
        let request = hyper::Request::builder()
            .method(this.request.method())
            .uri(format!("{}{}", this.base_address, this.request.uri()));

        let request = this
            .request
            .headers()
            .iter()
            .filter(|(header_name, _)| header_name != &hyper::header::HOST)
            .fold(request, |request, (header_name, header_value)| {
                request.header(header_name, header_value)
            });

        request.body(this.request.into_body()).map_err(Into::into)
    }
}

impl TryFrom<ProxyResponse> for hyper::Response<hyper::Body> {
    type Error = crate::Error;

    fn try_from(this: ProxyResponse) -> Result<Self, Self::Error> {
        let response = hyper::Response::builder().status(this.response.status());

        let response = this.response.headers().iter().fold(
            response,
            |response, (header_name, header_value)| match (header_name, header_value) {
                (&hyper::header::LOCATION, _)
                    if header_value
                        .to_str()
                        .unwrap()
                        .starts_with(&this.base_address) =>
                {
                    response.header(
                        header_name,
                        &header_value.as_ref()[this.base_address.len()..],
                    )
                }
                (_, _) => response.header(header_name, header_value),
            },
        );

        response.body(this.response.into_body()).map_err(Into::into)
    }
}

impl Default for Regex {
    fn default() -> Self {
        Self {
            name_manifest_reference: regex::Regex::new(
                r"/v2/(?P<name>.*)/manifests/(?P<reference>.*)",
            )
            .unwrap(),
        }
    }
}
