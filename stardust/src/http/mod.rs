pub mod utils;
pub mod session;
pub mod traceid;

use anyhow::anyhow;
use axum::http::Response;

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
