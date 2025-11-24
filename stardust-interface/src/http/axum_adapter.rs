use axum::{
    body::Body,
    http::{
        HeaderValue, Request, Response, StatusCode,
        header::{self, CONTENT_LENGTH, CONTENT_TYPE},
    },
    middleware::Next,
    response::IntoResponse,
};

use crate::http::utils::{into_string, is_json_content};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T: serde::Serialize> {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn with(data: T) -> Self {
        Self {
            code: 200,
            message: None,
            data: Some(data),
        }
    }

    pub fn into_json_string(self) -> stardust_common::Result<String> {
        let body = serde_json::to_string(&self)
            .map_err(|e| stardust_common::Error::ParseError(e.into()))?;
        Ok(body)
    }
}

impl ApiResponse<()> {
    pub fn ok() -> Self {
        Self {
            code: 200,
            message: None,
            data: None,
        }
    }
    pub fn code(code: StatusCode) -> Self {
        Self {
            code: code.as_u16(),
            message: None,
            data: None,
        }
    }

    pub fn error(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code: code.as_u16(),
            message: Some(message.into()),
            data: None,
        }
    }
}

impl<T: serde::Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap();
        axum::response::Response::builder()
            .status(StatusCode::from_u16(self.code).unwrap())
            .header(CONTENT_TYPE, "application/json")
            .header(CONTENT_LENGTH, body.len())
            .body(Body::from(body))
            .unwrap()
    }
}

impl From<stardust_common::Error> for ApiResponse<()> {
    fn from(value: stardust_common::Error) -> Self {
        let statuscode = match value {
            stardust_common::Error::AlreadyExists => StatusCode::CONFLICT,
            stardust_common::Error::Unauthorized => StatusCode::UNAUTHORIZED,
            stardust_common::Error::Forbidden => StatusCode::FORBIDDEN,
            stardust_common::Error::NotFound => StatusCode::NOT_FOUND,
            stardust_common::Error::InvalidParameter(_) => StatusCode::BAD_REQUEST,
            stardust_common::Error::Timeout => StatusCode::REQUEST_TIMEOUT,
            stardust_common::Error::Expired(_) => StatusCode::UNAUTHORIZED,
            stardust_common::Error::Duplicate(_) => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self::error(statuscode, value.to_string())
    }
}

pub async fn run(
    config: &stardust_common::config::ServerConfig,
    router: axum::Router,
) -> stardust_common::Result<()> {
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.host.as_str(), config.port)).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

pub async fn map_response(request: Request<Body>, next: Next) -> impl IntoResponse {
    let response = next.run(request).await;
    match response.status() {
        s if s == StatusCode::UNPROCESSABLE_ENTITY || s == StatusCode::UNSUPPORTED_MEDIA_TYPE => {
            if !is_json_content(response.headers()) {
                let (mut parts, body) = response.into_parts();
                let bodystr = into_string(body).await.unwrap_or_else(|e| {
                    tracing::warn!("Failed to read response body: {}", e);
                    String::new()
                });
                let content =
                    ApiResponse::error(s, bodystr).into_json_string().unwrap_or_else(|e| {
                        tracing::warn!("Failed to serialize error response: {}", e);
                        String::from(r#"{"code":500,"message":"into_json_string failed"}"#)
                    });
                parts.headers.extend([
                    (
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("application/json"),
                    ),
                    (
                        header::CONTENT_LENGTH,
                        HeaderValue::from(content.len() as u64),
                    ),
                ]);
                return Response::from_parts(parts, Body::from(content));
            }
        }
        _ => {}
    }
    response
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        handler::HandlerWithoutStateExt,
        http::{Request, StatusCode},
    };
    use stardust_common::config::HttpConfig;
    use tower::ServiceExt;
    use tower_http::{services::ServeDir, trace::TraceLayer};
    use tower_sessions::MemoryStore;
    use tracing::info_span;

    use crate::http::{TraceIdLayer, session_layer};

    fn setup_router(config: &Option<HttpConfig>) -> axum::Router {
        let router = axum::Router::new()
            .route(
                "/",
                axum::routing::get(async move || {
                    tracing::info!("Hello, World!");
                    "Hello, World!"
                }),
            )
            .layer(session_layer(MemoryStore::default()))
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &axum::extract::Request| {
                    info_span!(
                        "http.request",
                        method = %request.method(),
                        uri = %request.uri().path()
                    )
                }),
            )
            .layer(TraceIdLayer::default());

        let notfound = || async { (StatusCode::NOT_FOUND, "Not found") };
        if let Some(config) = config {
            let fallback_service = ServeDir::new(config.static_dir.as_str())
                .not_found_service(notfound.into_service());
            router.fallback_service(fallback_service)
        } else {
            router.fallback_service(notfound.into_service())
        }
    }

    #[tokio::test]
    async fn test_router() {
        let config = stardust_common::config::Config::test_config();
        stardust_common::logging::init(&config.logging);
        let router = setup_router(&config.server.http);
        let reqeust = Request::builder().method("GET").uri("/").body(Body::from("")).unwrap();
        router.oneshot(reqeust).await.unwrap();
    }
}
