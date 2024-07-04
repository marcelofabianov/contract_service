fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/gprc/service")
        .compile(&["proto/customer.proto"], &["proto"])?;
    Ok(())
}
