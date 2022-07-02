mod organization_integration_import_post;
pub(crate) mod organization_projects_post;

#[derive(Clone)]
pub(crate) struct Api {
    base_address: String,
    api_key: String,
    organization_id: String,
    integration_id: String,
}

#[derive(Debug)]
pub(crate) struct ApiError<T>(T);

impl<T: std::fmt::Debug> std::error::Error for ApiError<T> {}

impl<T: std::fmt::Debug> std::fmt::Display for ApiError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Api {
    /// Creates a new `Api` instance.
    pub(crate) fn new(
        base_address: impl Into<String>,
        api_key: impl Into<String>,
        organization_id: impl Into<String>,
        integration_id: impl Into<String>,
    ) -> Api {
        Api {
            base_address: base_address.into(),
            api_key: api_key.into(),
            organization_id: organization_id.into(),
            integration_id: integration_id.into(),
        }
    }

    /// Sends a organization integration import post request.
    ///
    /// A convenience method for sending a request to the backend.
    pub(crate) async fn send_organization_integration_import_post(
        &self,
        client: &crate::http::Client,
        name: impl Into<String>,
    ) -> crate::Result<organization_integration_import_post::Response> {
        use organization_integration_import_post::{Request, RequestBody, RequestBodyTarget};

        let request = Request {
            base_address: self.base_address.clone(),
            api_key: self.api_key.clone(),
            organization_id: self.organization_id.clone(),
            integration_id: self.integration_id.clone(),
            body: RequestBody {
                target: RequestBodyTarget { name: name.into() },
            },
        };

        client.request(request.try_into()?).await?.try_into()
    }

    /// Sends a organization projects post request.
    ///
    /// A convenience method for sending a request to the backend.
    pub(crate) async fn send_organization_projects_post(
        &self,
        client: &crate::http::Client,
        name: impl Into<String>,
    ) -> crate::Result<organization_projects_post::Response> {
        use organization_projects_post::{Request, RequestBody, RequestBodyFilters, Response};

        let request = Request {
            base_address: self.base_address.clone(),
            api_key: self.api_key.clone(),
            organization_id: self.organization_id.clone(),
            body: RequestBody {
                filters: RequestBodyFilters { name: name.into() },
            },
        };

        let response = client.request(request.try_into()?).await?;

        Response::try_from_response(response).await
    }
}
