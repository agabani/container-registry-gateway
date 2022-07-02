use super::ApiError;

pub(crate) struct Request {
    pub(crate) base_address: String,
    pub(crate) api_key: String,
    pub(crate) organization_id: String,
    pub(crate) integration_id: String,
    pub(crate) body: RequestBody,
}

#[derive(serde::Serialize)]
pub(crate) struct RequestBody {
    pub(crate) target: RequestBodyTarget,
}

#[derive(serde::Serialize)]
pub(crate) struct RequestBodyTarget {
    pub(crate) name: String,
}

pub(crate) struct Response {}

impl TryFrom<Request> for hyper::Request<hyper::Body> {
    type Error = crate::Error;

    fn try_from(this: Request) -> Result<Self, Self::Error> {
        hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri(format!(
                "{}/api/v1/org/{}/integrations/{}/import",
                this.base_address, this.organization_id, this.integration_id
            ))
            .header(
                hyper::header::AUTHORIZATION,
                format!("token {}", this.api_key),
            )
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(hyper::body::Body::from(serde_json::to_vec(&this.body)?))
            .map_err(Into::into)
    }
}

impl TryFrom<hyper::Response<hyper::Body>> for Response {
    type Error = crate::Error;

    fn try_from(this: hyper::Response<hyper::Body>) -> Result<Self, Self::Error> {
        if this.status() != hyper::StatusCode::CREATED {
            return Err(Box::new(ApiError(this)));
        }

        Ok(Response {})
    }
}
