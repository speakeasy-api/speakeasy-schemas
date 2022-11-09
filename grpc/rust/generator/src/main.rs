use std::fs::canonicalize;

#[cfg(feature = "tokio02")]
use tonic_build03::configure;

#[cfg(feature = "tokio")]
use tonic_build::configure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure()
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
