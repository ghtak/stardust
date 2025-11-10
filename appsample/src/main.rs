use std::sync::Arc;

use axum::routing::{get, post};
use module_user::interface::UserServiceProvider;
use stardust_interface::http::{Json, Path};

mod app;
mod container;
mod error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct JsonRequest {
    pub name: String,
}

async fn json_handler(
    Json(v, _): Json<JsonRequest, error::ApiError>,
) -> String {
    println!("{:?}", v);
    format!("Hello, {}!", v.name)
}

async fn path_handler(Path(name, _): Path<i32, error::ApiError>) -> String {
    println!("{}", name);
    format!("Hello, {}!", name)
}

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

pub async fn run_server(ct: Arc<app::Container>) {
    stardust_interface::http::run(
        &ct.config.server,
        axum::Router::new()
            .route("/", get(|| async { "Stardust Root" }))
            .route("/json", post(json_handler))
            .route("/path/{name}", get(path_handler))
            .merge(module_user::interface::http::routes(ct.clone())),
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
