use std::fs::canonicalize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir(canonicalize("../lib/src")?)
        .compile(
            &[
                canonicalize("../../protos/registry/embedaccesstoken/embedaccesstoken.proto")?,
                canonicalize("../../protos/registry/ingest/ingest.proto")?,
            ],
            &[canonicalize("../../protos/registry")?],
        )?;
    Ok(())
}
