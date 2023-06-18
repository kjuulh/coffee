use dagger_sdk::HostDirectoryOptsBuilder;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client = dagger_sdk::connect().await?;

    let src = client.host().directory_opts(
        ".",
        HostDirectoryOptsBuilder::default()
            .include(vec!["ci/", "crates/", "Cargo*"])
            .build()?,
    );

    let rust_base = client
        .container()
        .from("rustlang/rust:nightly")
        .with_workdir("/app")
        .with_directory("/app", src.id().await?)
        .with_env_variable("CARGO_INCREMENTAL", "0")
        .with_mounted_cache("~/.cargo", client.cache_volume("cargo-base").id().await?)
        .with_mounted_cache(
            "target/",
            client.cache_volume("cargo-target-debug").id().await?,
        );

    let rust_test = rust_base.with_exec(vec!["cargo", "test", "-p", "coffee"]);
    let _ = rust_test.exit_code().await?;

    Ok(())
}
