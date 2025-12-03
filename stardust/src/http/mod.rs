pub mod session;
pub mod traceid;
pub mod utils;

use anyhow::anyhow;
use axum::{http::Response, response::IntoResponse};

pub async fn run_server(
    config: &crate::config::ServerConfig,
    router: axum::Router,
) -> crate::Result<()> {
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.host.as_str(),
        config.port
    ))
    .await
    .map_err(|e| anyhow!("tcp bind failed: {:?}", e))?;
    axum::serve(listener, router)
        .await
        .map_err(|e| anyhow!("http serve failed: {:?}", e))?;
    Ok(())
}

pub async fn map_response(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let response = next.run(request).await;

    match response.status() {
        s if s == axum::http::StatusCode::UNPROCESSABLE_ENTITY
            || s == axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE =>
        {
            if !utils::is_json(&response.headers()) {
                let (mut parts, body) = response.into_parts();
                // build custom content if require
                let mut content = String::new();
                content.push_str(r#"{"message":"#);
                content.push_str(
                    &utils::into_string(body).await.unwrap_or_else(|e| {
                        tracing::warn!("Failed to read response body: {:?}", e);
                        format!("Failed to read response body {:?}", e)
                    }),
                );
                content.push_str(r#"}"#);

                parts.headers.extend([
                    (
                        axum::http::header::CONTENT_TYPE,
                        axum::http::header::HeaderValue::from_static(
                            "application/json",
                        ),
                    ),
                    (
                        axum::http::header::CONTENT_LENGTH,
                        axum::http::header::HeaderValue::from(
                            content.len() as u64
                        ),
                    ),
                ]);
                return Response::from_parts(
                    parts,
                    axum::body::Body::from(content),
                );
            }
        }
        _ => {}
    }

    response
}

impl IntoResponse for crate::Error {
    fn into_response(self) -> axum::response::Response {
        match &self {
            crate::Error::InvalidParameter(_) => {
                (axum::http::StatusCode::BAD_REQUEST, format!("{:?}", self))
                    .into_response()
            }
            crate::Error::IllegalState(_) => (
                axum::http::StatusCode::PRECONDITION_FAILED,
                format!("{:?}", self),
            )
                .into_response(),
            crate::Error::AlreadyExists(_) => {
                (axum::http::StatusCode::CONFLICT, format!("{:?}", self))
                    .into_response()
            }
            crate::Error::NotFound(_) => {
                (axum::http::StatusCode::NOT_FOUND, format!("{:?}", self))
                    .into_response()
            }
            crate::Error::Timeout => (
                axum::http::StatusCode::REQUEST_TIMEOUT,
                format!("{:?}", self),
            )
                .into_response(),
            crate::Error::Unauthorized => {
                (axum::http::StatusCode::UNAUTHORIZED, format!("{:?}", self))
                    .into_response()
            }
            crate::Error::Forbidden => {
                (axum::http::StatusCode::FORBIDDEN, format!("{:?}", self))
                    .into_response()
            }
            _ => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:?}", self),
            )
                .into_response(),
        }
    }
}
