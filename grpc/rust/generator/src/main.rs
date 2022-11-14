use std::fs::canonicalize;

#[cfg(feature = "tokio02")]
use prost_build08::Config;

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

#[cfg(feature = "tokio02")]
fn configure() -> tonic_build05::Builder {
    tonic_build05::configure().build_client(false)
}

#[cfg(feature = "tokio")]
fn configure() -> tonic_build::Builder {
    tonic_build::configure()
}

#[cfg(feature = "tokio02")]
fn transform() {
    transform_file("../speakeasy-protos-tokio-02/src/ingest.rs");
    transform_file("../speakeasy-protos-tokio-02/src/embedaccesstoken.rs");
}

#[cfg(feature = "tokio02")]
fn transform_file(file_path: &str) {
    let file = std::fs::read_to_string(file_path).unwrap();

    let transformed_file = file.replace("::prost::alloc", "::std");

    std::fs::write(file_path, transformed_file).unwrap();
}

#[cfg(feature = "tokio02")]
fn config() -> Config {
    let mut config = Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");
    config
}

#[cfg(feature = "tokio")]
fn config() -> Config {
    Config::new()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure()
        .build_server(false)
        .out_dir(canonicalize(out_dir())?)
        .compile_with_config(
            config(),
            &[
                canonicalize("../../protos/registry/embedaccesstoken/embedaccesstoken.proto")?,
                canonicalize("../../protos/registry/ingest/ingest.proto")?,
            ],
            &[canonicalize("../../protos/registry")?],
        )?;

    #[cfg(feature = "tokio02")]
    transform();

    Ok(())
}
