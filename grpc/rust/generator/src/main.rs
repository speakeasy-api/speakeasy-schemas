use std::fs::canonicalize;

#[cfg(feature = "tokio02")]
use tonic_build05::configure;
#[cfg(feature = "tokio02")]
use prost_build08::Config;

#[cfg(feature = "tokio")]
use tonic_build::configure;
#[cfg(feature = "tokio")]
use prost_build::Config;

#[cfg(feature = "tokio02")]
fn out_dir() -> &'static str {
    "../speakeasy-protos-tokio-02/src/"
}

#[cfg(feature = "tokio")]
fn out_dir() -> &'static str {
    "../speakeasy-protos-tokio-latest/src/"
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();

    #[cfg(feature = "tokio02")]
    config.protoc_arg("--experimental_allow_proto3_optional");

    configure()
        .build_server(false)
        .out_dir(canonicalize(out_dir())?)
        .compile_with_config(
            config,
            &[
                canonicalize("../../protos/registry/embedaccesstoken/embedaccesstoken.proto")?,
                canonicalize("../../protos/registry/ingest/ingest.proto")?,
            ],
            &[canonicalize("../../protos/registry")?],
        )?;
    Ok(())
}
