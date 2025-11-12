use axum::{body::Body, http::StatusCode, response::IntoResponse};

pub trait CommonErrorToResponse {
    fn into_response(error: stardust_common::Error)
    -> axum::response::Response;
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T: serde::Serialize> {
    pub code: u16,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 200,
            message: None,
            data: Some(data),
        }
    }
}

impl<T: serde::Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap();
        let content_length = body.len();
        let mut response = axum::response::Response::new(Body::from(body));
        *response.status_mut() = StatusCode::from_u16(self.code).unwrap();
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("application/json"),
        );
        response.headers_mut().insert(
            axum::http::header::CONTENT_LENGTH,
            axum::http::HeaderValue::from_str(&content_length.to_string())
                .unwrap(),
        );
        response
    }
}

pub async fn run(
    config: &stardust_common::config::ServerConfig,
    router: axum::Router,
) -> stardust_common::Result<()> {
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.host.as_str(),
        config.port
    ))
    .await?;
    axum::serve(listener, router).await?;
    Ok(())
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
            .layer(TraceLayer::new_for_http().make_span_with(
                |request: &axum::extract::Request| {
                    info_span!(
                        "http.request",
                        method = %request.method(),
                        uri = %request.uri().path()
                    )
                },
            ))
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
        let reqeust = Request::builder()
            .method("GET")
            .uri("/")
            .body(Body::from(""))
            .unwrap();
        router.oneshot(reqeust).await.unwrap();
    }
}
