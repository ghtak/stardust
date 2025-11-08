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

#[tokio::main]
async fn main() {
    let config = stardust_common::config::Config::test_config();
    println!("{:?}", config);
    stardust_inbound::http::run(
        &config.server,
        axum::Router::new()
            .route("/", get(|| async { "Stardust Root" }))
            .route("/json", post(json_handler))
            .route("/path/{name}", get(path_handler))
    )
    .await
    .unwrap();
}
