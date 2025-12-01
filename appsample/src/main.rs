use std::sync::Arc;

use axum::{handler::HandlerWithoutStateExt, http::StatusCode};
use module_user::Container;
use stardust_core::repository::MigrationRepository;
use stardust_core::service::MigrationService;
use stardust_db::database::Database;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::info_span;

use crate::{app::UserMigrationServiceImpl, container::AppContainer};

mod app;
mod container;

fn need_migration() -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "migrate" {
        true
    } else {
        false
    }
}

async fn migration(ct: Arc<AppContainer>) -> stardust_common::Result<()> {
    let migration_repo = Arc::new(app::MigrationRepositoryImpl::new());
    match migration_repo.create_table(&mut ct.database.handle()).await {
        Ok(_) => println!("Migration successful"),
        Err(e) => eprintln!("Migration failed: {}", e),
    };

    let user_migration = UserMigrationServiceImpl::new(
        ct.database.clone(),
        Arc::new(app::UserMigrationRepositoryImpl::new()),
        ct.user_service(),
        migration_repo.clone(),
    );
    match user_migration.migrate().await {
        Ok(_) => println!("User module migration successful"),
        Err(e) => eprintln!("User module migration failed: {}", e),
    }

    let oauth2_migration = app::OAuth2MigrationServiceImpl::new(
        ct.database.clone(),
        Arc::new(app::OAuth2MigrationRepositoryImpl::new()),
        migration_repo.clone(),
    );
    match oauth2_migration.migrate().await {
        Ok(_) => println!("OAuth2 module migration successful"),
        Err(e) => eprintln!("OAuth2 module migration failed: {}", e),
    }

    Ok(())
}

pub fn with_fallback_service(
    router: axum::Router,
    httpcfg: &Option<stardust_common::config::HttpConfig>,
) -> axum::Router {
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

    if let Some(httpcfg) = httpcfg {
        router.fallback_service(
            ServeDir::new(httpcfg.static_dir.as_str()).not_found_service(notfound.into_service()),
        )
    } else {
        router.fallback_service(notfound.into_service())
    }
}

pub async fn new_router(routers: Vec<axum::Router>) -> axum::Router {
    let mut router = axum::Router::new();
    for r in routers {
        router = router.merge(r);
    }
    router
        .layer(stardust_interface::http::session_layer(
            tower_sessions::MemoryStore::default(),
        ))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &axum::extract::Request| {
                info_span!(
                    "http.request",
                    method = %request.method(),
                    path = %request.uri().path(),
                )
            }),
        )
        .layer(stardust_interface::http::TraceIdLayer::default())
        .layer(axum::middleware::from_fn(
            stardust_interface::http::map_response,
        ))
}

#[tokio::main]
async fn main() {
    let config = stardust_common::config::Config::test_config();
    stardust_common::logging::init(&config.logging);
    tracing::info!("config: {:?}", config);
    stardust_core::audit(0, "sys.init", serde_json::Value::Null);

    let app_container = AppContainer::build(config).await.unwrap();

    if need_migration() {
        migration(app_container.clone()).await.unwrap();
    }

    let app_router = new_router(vec![
        module_user::interface::http::routes(app_container.clone()),
        module_oauth2_server::interface::http::routes(app_container.clone()),
    ])
    .await;
    let app_router = with_fallback_service(app_router, &app_container.config.server.http);
    stardust_interface::http::run(&app_container.config.server, app_router).await.unwrap();
}
