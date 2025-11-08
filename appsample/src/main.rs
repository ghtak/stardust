use axum::routing::{get, post};
use stardust_inbound::http::{Json, Path};
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

pub async fn run_http() {
    let config = stardust_common::config::Config::test_config();
    println!("{:?}", config);
    stardust_inbound::http::run(
        &config.server,
        axum::Router::new()
            .route("/", get(|| async { "Stardust Root" }))
            .route("/json", post(json_handler))
            .route("/path/{name}", get(path_handler)),
    )
    .await
    .unwrap();
}

async fn migration(
    database: stardust_db::Database,
) -> stardust_common::Result<()> {
    stardust_core::migration::migrate(database.clone()).await.unwrap();
    module_user::infra::migration::migrate(database.clone()).await.unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    let config = stardust_common::config::Config::test_config();
    stardust_common::logging::init(&config.logging);
    let database = stardust_db::Database::open(&config.database).await.unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "migrate" {
            migration(database.clone()).await.unwrap();
        }
    }
    //run_http().await;
}
