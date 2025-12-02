#[tokio::main]
async fn main() {
    let config = stardust::config::Config::test_config();
    stardust::logging::init(&config.logging);

    let database =
        stardust::database::internal::postgres::Database::new(&config.database)
            .await
            .unwrap();

    stardust::infra::migration::init(database.clone()).await.unwrap();
    module_user::infra::migration::migrate(database.clone()).await.unwrap();

    stardust::http::run_server(
        &config.server,
        axum::Router::new()
            .route("/hello", axum::routing::get(|| async { "Hello, World!" })),
    )
    .await
    .unwrap();
}
