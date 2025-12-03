// use anyhow::anyhow;

// const FILE_DESCRIPTOR_SET: &[u8] =
//     tonic::include_file_descriptor_set!("FILE_DESCRIPTOR_SET");

// pub async fn run_server(
//     _config: &crate::config::ServerConfig,
// ) -> crate::Result<()> {
//     // let addr =
//     //         format!("{}:{}", config.host.as_str(), config.port).parse()?;
//     let _reflection_service = tonic_reflection::server::Builder::configure()
//         .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
//         .build_v1alpha()
//         .map_err(|e| anyhow!("build tonic reflection error: {:?}", e))?;
//     // Server::builder()
//     //     // .add_service(GreeterServer::new(GreeterImpl {
//     //     //     container: self.container.clone(),
//     //     // }))
//     //     .add_service(reflection_service)
//     //     .serve(addr)
//     //     .await
//     //     .map_err(|e| anyhow!("run tonic error: {:?}", e))?;
//     Ok(())
// }
