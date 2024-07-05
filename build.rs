use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path("src/grpc/pb/descriptor.bin")
        .compile(&["src/proto/customer.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    let out_dir = env::var("OUT_DIR")?;
    let descriptor_path = Path::new(&out_dir).join("descriptor.bin");
    fs::copy("src/grpc/pb/descriptor.bin", descriptor_path)?;

    Ok(())
}
