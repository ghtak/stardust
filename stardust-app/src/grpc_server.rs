pub mod hello {
    tonic::include_proto!("hello");
}
use hello::greeter_server::{Greeter, GreeterServer};
use hello::{HelloRequest, HelloResponse};
use tonic::{Request, Response, Status, transport::Server};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = HelloResponse {
            message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> stardust::Result<()> {
    let config = stardust::config::Config::test_config();
    let addr =
        format!("{}:{}", config.server.host.as_str(), config.server.port)
            .parse()
            .map_err(|e| anyhow::anyhow!("address parse error {:?}", e))?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await
        .map_err(|e| anyhow::anyhow!("run tonic error: {:?}", e))?;
    Ok(())
}
