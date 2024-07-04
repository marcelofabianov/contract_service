fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("./src/grpc/service")
        .compile(&["./proto/customer.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    Ok(())
}
