fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("./src")
        .compile(
            &[
                "../protos/registry/embedaccesstoken/embedaccesstoken.proto",
                "../protos/registry/ingest/ingest.proto",
            ],
            &["../protos/registry"],
        )?;
    Ok(())
}
