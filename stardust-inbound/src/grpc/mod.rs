/*
---
syntax = "proto3";

package hello;

message HelloRequest {
  string name = 1;
}

message HelloResponse {
  string message = 1;
}

service Greeter{
  rpc SayHello(HelloRequest) returns (HelloResponse);
}
---
pub mod hello {
    tonic::include_proto!("hello");
}

mod tonic_adapter;

pub use tonic_adapter::TonicAdapter;

---

use std::sync::Arc;

use anyhow::Context;
use tonic::{Request, Response, Status, service::Routes, transport::Server};

use crate::{
    app::{
        Container,
        interface::grpc::hello::{
            HelloRequest, HelloResponse,
            greeter_server::{Greeter, GreeterServer},
        },
    },
    common,
    common::Error,
};

pub struct TonicAdapter {
    container: Arc<Container>,
}

const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("FILE_DESCRIPTOR_SET");

impl TonicAdapter {
    pub fn new(container: Arc<Container>) -> Self {
        TonicAdapter { container }
    }

    pub fn axum_router(&self) -> common::Result<axum::Router> {
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build_v1alpha()
            .context("build tonic reflection error")
            .map_err(common::Error::TonicError)?;
        Ok(Routes::default()
            .add_service(GreeterServer::new(GreeterImpl {
                container: self.container.clone(),
            }))
            .add_service(reflection_service)
            .into_axum_router())
    }

    pub async fn run(
        &self,
        config: &common::ServerConfig,
    ) -> Result<(), Error> {
        let addr =
            format!("{}:{}", config.host.as_str(), config.port).parse()?;
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build_v1alpha()
            .context("build tonic reflection error")
            .map_err(Error::TonicError)?;
        Server::builder()
            .add_service(GreeterServer::new(GreeterImpl {
                container: self.container.clone(),
            }))
            .add_service(reflection_service)
            .serve(addr)
            .await
            .context("run tonic error")
            .map_err(Error::TonicError)?;
        Ok(())
    }
}

pub struct GreeterImpl {
    container: Arc<Container>,
}

#[tonic::async_trait]
impl Greeter for GreeterImpl {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let resp = HelloResponse {
            message: format!(
                "Hello! {} {:?}",
                request.into_inner().name,
                self.container.database.pool()
            ),
        };
        Ok(Response::new(resp))
    }
}

---
build.rs

use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_prost_build::compile_protos("src/interfaces/grpc/proto/hello.proto")?;
    // Ok(())
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("FILE_DESCRIPTOR_SET.bin"))
        .compile_protos(
            &["src/app/interface/grpc/proto/hello.proto"],
            &["src/app/interface/grpc/proto"],
        )?;
    Ok(())
}


*/