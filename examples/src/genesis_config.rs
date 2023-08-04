#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;

    // NOTE: this API is under the "experimental" flag.
    let genesis_config = worker.genesis_config().await?;

    // dump the genesis config info to stdout
    println!("Genesis Config {:?}", genesis_config);
    Ok(())
}
