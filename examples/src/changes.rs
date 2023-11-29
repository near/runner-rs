use serde_json::json;

const STATUS_MSG_WASM_FILEPATH: &str = "./examples/res/status_message.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = near_workspaces::sandbox().await?;
    let wasm = std::fs::read(STATUS_MSG_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    _ = contract
        .call("set_status")
        .args_json(json!({
            "message": "hello_world",
        }))
        .transact()
        .await?
        .into_result()?;

    // NOTE: this API is under the "experimental" flag and no guarantees are given.
    let res = worker.changes(&[contract.id().clone()]).await?;

    // Example output:
    //
    // StateChangesInBlock RpcStateChangesInBlockResponse {
    //     block_hash: 5SnL82tfQX1NtsSuqU5334ThZxM1B5KkUWUbeeMvVNRH,
    //     changes: [],
    // }
    println!("StateChangesInBlock {res:#?}");
    Ok(())
}
