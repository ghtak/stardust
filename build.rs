// use std::{env, path::PathBuf};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // tonic_prost_build::compile_protos("src/interfaces/grpc/proto/hello.proto")?;
//     // Ok(())
//     let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
//     tonic_prost_build::configure()
//         .build_server(true)
//         .build_client(true)
//         .file_descriptor_set_path(out_dir.join("FILE_DESCRIPTOR_SET.bin"))
//         .compile_protos(
//             &["stardust/src/grpc/proto/hello.proto"],
//             &["stardust/src/grpc/proto"],
//         )?;
//     Ok(())
// }
