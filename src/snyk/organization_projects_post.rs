use super::ApiError;

pub(crate) struct Request {
    pub(crate) base_address: String,
    pub(crate) api_key: String,
    pub(crate) organization_id: String,
    pub(crate) body: RequestBody,
}

#[derive(serde::Serialize)]
pub(crate) struct RequestBody {
    pub(crate) filters: RequestBodyFilters,
}

#[derive(serde::Serialize)]
pub(crate) struct RequestBodyFilters {
    pub(crate) name: String,
}

pub(crate) struct Response {
    pub(crate) body: ResponseBody,
}

#[derive(serde::Deserialize)]
pub(crate) struct ResponseBody {
    pub(crate) projects: Vec<ResponseBodyProject>,
}

#[derive(serde::Deserialize)]
pub(crate) struct ResponseBodyProject {
    #[allow(dead_code)]
    pub(crate) name: String,
    pub(crate) attributes: ResponseBodyProjectAttributes,
    #[serde(rename = "issueCountsBySeverity")]
    pub(crate) issue_counts_by_severity: ResponseBodyProjectIssueCountsBySeverity,
}

#[derive(serde::Deserialize)]
pub(crate) struct ResponseBodyProjectAttributes {
    pub(crate) criticality: Vec<String>,
}

#[derive(serde::Deserialize)]
pub(crate) struct ResponseBodyProjectIssueCountsBySeverity {
    pub(crate) critical: u32,
    pub(crate) low: u32,
    pub(crate) high: u32,
    pub(crate) medium: u32,
}

impl TryFrom<Request> for hyper::Request<hyper::Body> {
    type Error = crate::Error;

    fn try_from(this: Request) -> Result<Self, Self::Error> {
        hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri(format!(
                "{}/api/v1/org/{}/projects",
                this.base_address, this.organization_id
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

impl Response {
    pub(crate) async fn try_from_response(
        this: hyper::Response<hyper::Body>,
    ) -> crate::Result<Response> {
        use hyper::body::Buf;

        if this.status() != hyper::StatusCode::OK {
            return Err(Box::new(ApiError(this)));
        }

        let buffer = hyper::body::aggregate(this.into_body()).await?;

        let body = serde_json::from_reader(buffer.reader())?;

        Ok(Response { body })
    }
}
