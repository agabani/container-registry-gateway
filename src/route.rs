use axum::{extract::Path, http::status::StatusCode, Extension};

use crate::{oci, state::State};

/// GET /health/liveness
///
/// Returns 200 if the server is healthy.
///
/// This endpoint is used by the Kubernetes liveness probe.
#[allow(clippy::unused_async)]
pub(crate) async fn health_liveness_get() -> StatusCode {
    StatusCode::OK
}

/// GET /health/readiness
///
/// Returns 200 if the server is ready.
///
/// This endpoint is used by the Kubernetes readiness probe.
#[allow(clippy::unused_async)]
pub(crate) async fn health_readiness_get() -> StatusCode {
    StatusCode::OK
}

/// Router for /v2/* nested routes
///
/// This router is used by the OCI distribution specification proxy.
///
/// This router has been implemented manually as a workaround for the `axum::routing::Router` not supporting
/// both captures and wildcards at the same time.
pub(crate) async fn v2_routes(
    state: Extension<State>,
    request: axum::http::Request<axum::body::Body>,
) -> Result<hyper::Response<hyper::Body>, StatusCode> {
    let uri = request.uri().to_string();

    let name_manifest_reference = state.oci_regex.name_manifest_reference.captures(&uri);

    match (request.method(), name_manifest_reference) {
        (&(axum::http::Method::GET | axum::http::Method::HEAD), Some(captures)) => {
            v2_name_manifest_reference_get_head(
                &state,
                Path((
                    captures["name"].to_string(),
                    captures["reference"].to_string(),
                )),
                request,
            )
            .await
        }
        (&axum::http::Method::PUT, Some(captures)) => {
            v2_name_manifest_reference_put(
                &state,
                Path((
                    captures["name"].to_string(),
                    captures["reference"].to_string(),
                )),
                request,
            )
            .await
        }
        _ => v2_proxy(&state, request).await,
    }
}

/// GET|HEAD /v2/:name/manifests/:reference
///
/// This endpoint is used by the OCI distribution specification proxy.
#[tracing::instrument(skip(state, request))]
pub(crate) async fn v2_name_manifest_reference_get_head(
    state: &Extension<State>,
    Path((name, reference)): Path<(String, String)>,
    request: axum::http::Request<axum::body::Body>,
) -> Result<hyper::Response<hyper::Body>, StatusCode> {
    let response = state
        .snyk_api
        .send_organization_projects_post(&state.http_client, format!("{}:{}", name, reference))
        .await
        .map_err(|error| {
            tracing::error!(?error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if let Some(project) = response.body.projects.first() {
        let criticality = &project.attributes.criticality;
        let issue_count = &project.issue_counts_by_severity;
        let project_criticality = if criticality.contains(&"critical".to_string()) {
            4
        } else if criticality.contains(&"high".to_string()) {
            3
        } else if criticality.contains(&"medium".to_string()) {
            2
        } else if criticality.contains(&"low".to_string()) {
            1
        } else {
            0
        };
        if issue_count.critical > 0 && project_criticality < 4 {
            let body = serde_json::to_vec(&oci::Response {
                errors: vec![oci::ResponseError {
                    code: "DENIED".to_string(),
                    message: "Image exceeded vulnerability threshold critical".to_string(),
                    details: None,
                }],
            })
            .map_err(|error| {
                tracing::error!(?error);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            return hyper::Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(hyper::Body::from(body))
                .map_err(|error| {
                    tracing::error!(?error);
                    StatusCode::INTERNAL_SERVER_ERROR
                });
        } else if issue_count.high > 0 && project_criticality < 3 {
            let body = serde_json::to_vec(&oci::Response {
                errors: vec![oci::ResponseError {
                    code: "DENIED".to_string(),
                    message: "Image exceeded vulnerability threshold high".to_string(),
                    details: None,
                }],
            })
            .map_err(|error| {
                tracing::error!(?error);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            return hyper::Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(hyper::Body::from(body))
                .map_err(|error| {
                    tracing::error!(?error);
                    StatusCode::INTERNAL_SERVER_ERROR
                });
        } else if issue_count.medium > 0 && project_criticality < 2 {
            let body = serde_json::to_vec(&oci::Response {
                errors: vec![oci::ResponseError {
                    code: "DENIED".to_string(),
                    message: "Image exceeded vulnerability threshold medium".to_string(),
                    details: None,
                }],
            })
            .map_err(|error| {
                tracing::error!(?error);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            return hyper::Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(hyper::Body::from(body))
                .map_err(|error| {
                    tracing::error!(?error);
                    StatusCode::INTERNAL_SERVER_ERROR
                });
        } else if issue_count.low > 0 && project_criticality < 1 {
            let body = serde_json::to_vec(&oci::Response {
                errors: vec![oci::ResponseError {
                    code: "DENIED".to_string(),
                    message: "Image exceeded vulnerability threshold low".to_string(),
                    details: None,
                }],
            })
            .map_err(|error| {
                tracing::error!(?error);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            return hyper::Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(hyper::Body::from(body))
                .map_err(|error| {
                    tracing::error!(?error);
                    StatusCode::INTERNAL_SERVER_ERROR
                });
        }
    } else {
        let body = serde_json::to_vec(&oci::Response {
            errors: vec![oci::ResponseError {
                code: "DENIED".to_string(),
                message: "Image not monitored for vulnerabilities".to_string(),
                details: None,
            }],
        })
        .map_err(|error| {
            tracing::error!(?error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        return hyper::Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(hyper::Body::from(body))
            .map_err(|error| {
                tracing::error!(?error);
                StatusCode::INTERNAL_SERVER_ERROR
            });
    }

    v2_proxy(state, request).await
}

/// PUT /v2/:name/manifests/:reference
///
/// This endpoint is used by the OCI distribution specification proxy.
#[tracing::instrument(skip(state, request))]
pub(crate) async fn v2_name_manifest_reference_put(
    state: &Extension<State>,
    Path((name, reference)): Path<(String, String)>,
    request: axum::http::Request<axum::body::Body>,
) -> Result<hyper::Response<hyper::Body>, StatusCode> {
    let response = v2_proxy(state, request).await;

    state
        .snyk_api
        .send_organization_integration_import_post(
            &state.http_client,
            format!("{}:{}", name, reference),
        )
        .await
        .map_err(|error| {
            tracing::error!(?error);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    response
}

/// Fallback route for /v2/* if no routes match the incoming request.
///
/// This endpoint is used by the OCI distribution specification proxy.
pub(crate) async fn v2_proxy(
    state: &Extension<State>,
    request: axum::http::Request<axum::body::Body>,
) -> Result<hyper::Response<hyper::Body>, StatusCode> {
    state
        .oci_proxy
        .send(&state.http_client, request)
        .await
        .map_err(|error| {
            tracing::error!(?error);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
