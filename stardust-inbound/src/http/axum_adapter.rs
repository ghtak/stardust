
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
