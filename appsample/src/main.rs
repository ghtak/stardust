use std::{sync::Arc, usize};

use axum::{
    body::Body,
    handler::HandlerWithoutStateExt,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use module_user::interface::UserServiceProvider;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info_span;

mod app;
mod container;
mod error;

async fn build_container() -> Arc<app::Container> {
    let config = stardust_common::config::Config::test_config();
    stardust_common::logging::init(&config.logging);
    tracing::info!("config: {:?}", config);
    let database = stardust_db::Database::open(&config.database).await.unwrap();
    let hasher = Arc::new(app::HasherImpl::default());
    let user_service =
        Arc::new(app::UserServiceImpl::new(database.clone(), hasher.clone()));
    let container = app::Container::new(config, database, user_service);
    Arc::new(container)
}

async fn migration(ct: Arc<app::Container>) -> stardust_common::Result<()> {
    match stardust_core::migration::migrate(ct.database.clone()).await {
        Ok(_) => println!("Migration successful"),
        Err(e) => eprintln!("Migration failed: {}", e),
    };

    match module_user::infra::migration::migrate(
        ct.database.clone(),
        ct.user_service(),
    )
    .await
    {
        Ok(_) => println!("User module migration successful"),
        Err(e) => eprintln!("User module migration failed: {}", e),
    }
    Ok(())
}

pub async fn map_response(
    request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let response = next.run(request).await;
    match response.status() {
        StatusCode::UNPROCESSABLE_ENTITY => {
            let is_json = response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .map(|ct| ct.contains("application/json"))
                .unwrap_or(false);
            if !is_json {
                let (mut parts, body) = response.into_parts();
                let bytes =
                    axum::body::to_bytes(body, usize::MAX).await.unwrap();
                let message = String::from_utf8_lossy(bytes.as_ref());
                let response_body = Body::from(
                    serde_json::json!({
                        "code": StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                        "message": message
                    })
                    .to_string(),
                );
                parts.headers.insert(
                    axum::http::header::CONTENT_TYPE,
                    axum::http::HeaderValue::from_static("application/json"),
                );
                return Response::from_parts(parts, response_body);
            }
        }
        _ => {}
    }
    response
}

pub async fn new_router(ct: Arc<app::Container>) -> axum::Router {
    let router = axum::Router::new()
        .merge(module_user::interface::http::routes(ct.clone()))
        .layer(stardust_interface::http::session_layer(
            tower_sessions::MemoryStore::default(),
        ))
        .layer(TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                info_span!(
                    "http.request",
                    method = %request.method(),
                    path = %request.uri().path(),
                )
            },
        ))
        .layer(stardust_interface::http::TraceIdLayer::default())
        .layer(axum::middleware::from_fn(map_response));

    let notfound = || async {
        (
            StatusCode::NOT_FOUND,
            axum::Json(serde_json::json!({
                        "error" : {
                            "code": StatusCode::NOT_FOUND.as_u16(),
                            "message": "Not Found"
                        }
            })),
        )
    };

    if let Some(httpcfg) = &ct.config.server.http {
        router.fallback_service(
            ServeDir::new(httpcfg.static_dir.as_str())
                .not_found_service(notfound.into_service()),
        )
    } else {
        router.fallback_service(notfound.into_service())
    }
}

pub async fn run_server(ct: Arc<app::Container>) {
    stardust_interface::http::run(
        &ct.config.server,
        new_router(ct.clone()).await,
    )
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    let ct = build_container().await;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "migrate" {
        migration(ct.clone()).await.unwrap();
    }
    run_server(ct).await;
}
