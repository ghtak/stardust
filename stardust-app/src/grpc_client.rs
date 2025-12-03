pub mod hello {
    tonic::include_proto!("hello");
}

use hello::HelloRequest;
use hello::greeter_client::GreeterClient;

#[tokio::main]
async fn main() -> stardust::Result<()> {
    let config = stardust::config::Config::test_config();
    let addr = format!("http://127.0.0.1:{}", config.server.port);
    let mut client = GreeterClient::connect(addr)
        .await
        .map_err(|e| anyhow::anyhow!("connect error {:?}", e))?;
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let response = client
        .say_hello(request)
        .await
        .map_err(|e| anyhow::anyhow!("say_hello error {:?}", e))?;
    println!("RESPONSE={:?}", response);
    Ok(())
}
