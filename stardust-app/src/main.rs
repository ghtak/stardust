use axum::{handler::HandlerWithoutStateExt, http::StatusCode};
use tower_http::services::ServeDir;

pub mod container;

#[tokio::main]
async fn main() {
    let config = stardust::config::Config::test_config();
    stardust::logging::init(&config.logging);

    let container = container::Container::build(config.clone()).await.unwrap();

    stardust::infra::migration::init(container.database.clone()).await.unwrap();
    module_user::infra::migration::migrate(
        container.database.clone(),
        container.clone(),
    )
    .await
    .unwrap();
    module_oauth2_server::infra::migration::migrate(container.database.clone())
        .await
        .unwrap();

    let router = axum::Router::new()
        .merge(module_user::interface::http::routes(container.clone()))
        .merge(module_oauth2_server::interface::http::routes(
            container.clone(),
        ))
        .layer(stardust::http::session::session_layer(
            tower_sessions::MemoryStore::default(),
        ))
        .layer(
            tower_http::trace::TraceLayer::new_for_http().make_span_with(
                |request: &axum::extract::Request| {
                    tracing::info_span!(
                        "http.request",
                        method = %request.method(),
                        path = %request.uri().path(),
                    )
                },
            ),
        )
        .layer(stardust::http::traceid::TraceIdLayer::default())
        .layer(axum::middleware::from_fn(stardust::http::map_response));

    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }
    let notfound = handle_404.into_service();

    let router = if let Some(httpcfg) = &config.server.http {
        router
            .nest_service(
                httpcfg.static_root.as_str(),
                ServeDir::new(httpcfg.static_dir.as_str()),
            )
            .fallback_service(notfound)
    } else {
        router.fallback_service(notfound)
    };

    stardust::http::run_server(&config.server, router).await.unwrap();
}
