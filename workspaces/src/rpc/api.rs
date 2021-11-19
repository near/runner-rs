use super::client;
use super::tool;
use super::types::{AccountInfo, NearBalance};

use anyhow::anyhow;
use std::collections::HashMap;
use std::path::Path;

use crate::runtime::context::MISSING_RUNTIME_ERROR;
use near_crypto::{InMemorySigner, KeyType, PublicKey, Signer};
use near_jsonrpc_client::methods::{
    self,
    sandbox_patch_state::{RpcSandboxPatchStateRequest, RpcSandboxPatchStateResponse},
};
use near_jsonrpc_primitives::types::query::{QueryResponseKind, RpcQueryRequest};
use near_primitives::borsh::BorshSerialize;
use near_primitives::state_record::StateRecord;
use near_primitives::transaction::SignedTransaction;
use near_primitives::types::{AccountId, Balance, Finality, FunctionArgs, Gas, StoreKey};
use near_primitives::views::{FinalExecutionOutcomeView, FinalExecutionStatus, QueryRequest};

pub(crate) const NEAR_BASE: Balance = 1_000_000_000_000_000_000_000_000;
const ERR_INVALID_VARIANT: &str =
    "Incorrect variant retrieved while querying: maybe a bug in RPC code?";
const DEV_ACCOUNT_SEED: &str = "testificate";
pub(crate) const DEFAULT_CALL_FN_GAS: Gas = 10000000000000;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CallExecutionResult {
    /// Execution status. Contains the result in case of successful execution.
    pub status: FinalExecutionStatus,
    /// Total gas burnt by the call execution
    pub total_gas_burnt: Gas,
}

impl From<FinalExecutionOutcomeView> for CallExecutionResult {
    fn from(transaction_result: FinalExecutionOutcomeView) -> Self {
        CallExecutionResult {
            status: transaction_result.status,
            total_gas_burnt: transaction_result.transaction_outcome.outcome.gas_burnt
                + transaction_result
                    .receipts_outcome
                    .iter()
                    .map(|t| t.outcome.gas_burnt)
                    .sum::<u64>(),
        }
    }
}

pub async fn display_account_info(account_id: AccountId) -> anyhow::Result<AccountInfo> {
    let query_resp = client::new()
        .call(&RpcQueryRequest {
            block_reference: Finality::Final.into(),
            request: QueryRequest::ViewAccount {
                account_id: account_id.clone(),
            },
        })
        .await?;

    let account_view = match query_resp.kind {
        QueryResponseKind::ViewAccount(result) => result,
        _ => return Err(anyhow!("Error call result")),
    };

    Ok(AccountInfo {
        account_id,
        block_height: query_resp.block_height,
        block_hash: query_resp.block_hash,
        balance: NearBalance::from_yoctonear(account_view.amount),
        stake: NearBalance::from_yoctonear(account_view.locked),
        used_storage_bytes: account_view.storage_usage,
    })
}

pub async fn transfer_near(
    signer: &dyn Signer,
    signer_id: AccountId,
    receiver_id: AccountId,
    amount_yocto: Balance,
) -> anyhow::Result<CallExecutionResult> {
    client::send_tx_and_retry(|| async {
        let (access_key, _, block_hash) =
            tool::access_key(signer_id.clone(), signer.public_key()).await?;

        Ok(SignedTransaction::send_money(
            access_key.nonce + 1,
            signer_id.clone(),
            receiver_id.clone(),
            signer,
            amount_yocto,
            block_hash,
        ))
    })
    .await
    .map(Into::into)
}

pub async fn call(
    signer: &dyn Signer,
    signer_id: AccountId,
    contract_id: AccountId,
    method_name: String,
    args: Vec<u8>,
    deposit: Option<Balance>,
) -> anyhow::Result<CallExecutionResult> {
    let signer = InMemorySigner::from_file(&tool::credentials_filepath(signer_id.clone()).unwrap());
    client::new()
        ._call(&signer, contract_id, method_name, args, None, deposit)
        .await
        .map(Into::into)
}

pub async fn view(
    contract_id: AccountId,
    method_name: String,
    args: FunctionArgs,
) -> anyhow::Result<serde_json::Value> {
    let query_resp = client::new()
        .call(&RpcQueryRequest {
            block_reference: Finality::None.into(),
            request: QueryRequest::CallFunction {
                account_id: contract_id,
                method_name,
                args,
            },
        })
        .await?;

    let call_result = match query_resp.kind {
        QueryResponseKind::CallResult(result) => result.result,
        _ => return Err(anyhow!("Error call result")),
    };

    let result = std::str::from_utf8(&call_result)?;
    Ok(serde_json::from_str(result)?)
}

pub async fn view_state(
    contract_id: AccountId,
    prefix: Option<StoreKey>,
) -> anyhow::Result<HashMap<String, Vec<u8>>> {
    client::retry(|| async {
        let query_resp = client::new()
            .call(&methods::query::RpcQueryRequest {
                block_reference: Finality::None.into(),
                request: QueryRequest::ViewState {
                    account_id: contract_id.clone(),
                    prefix: prefix.clone().unwrap_or_else(|| vec![].into()),
                },
            })
            .await?;

        match query_resp.kind {
            QueryResponseKind::ViewState(state) => tool::into_state_map(&state.values),
            _ => Err(anyhow!(ERR_INVALID_VARIANT)),
        }
    })
    .await
}

pub async fn patch_state<T>(
    account_id: AccountId,
    key: String,
    value: &T,
) -> Result<RpcSandboxPatchStateResponse, String>
where
    T: BorshSerialize,
{
    // Patch state only exists within sandbox
    crate::runtime::assert_within(&["sandbox"]);

    let value = T::try_to_vec(value).unwrap();
    let state = StateRecord::Data {
        account_id,
        data_key: key.into(),
        value,
    };
    let records = vec![state];

    let query_resp = client::new()
        .call(&RpcSandboxPatchStateRequest { records })
        .await
        .map_err(|err| format!("Failed to patch state: {:?}", err));

    query_resp
}

pub async fn create_account(
    signer: &dyn Signer,
    signer_id: AccountId,
    new_account_id: AccountId,
    new_account_pk: PublicKey,
    deposit: Option<Balance>,
) -> anyhow::Result<CallExecutionResult> {
    client::send_tx_and_retry(|| async {
        let (access_key, _, block_hash) =
            tool::access_key(signer_id.clone(), signer.public_key()).await?;

        Ok(SignedTransaction::create_account(
            access_key.nonce + 1,
            signer_id.clone(),
            new_account_id.clone(),
            deposit.unwrap_or(NEAR_BASE),
            new_account_pk.clone(),
            signer,
            block_hash,
        ))
    })
    .await
    .map(Into::into)
}

/// Creates a top level account. While in sandbox, we can grab the `ExecutionOutcomeView`, but
/// while in Testnet or Mainnet, a helper account creator is used instead which does not
/// provide the `ExecutionOutcomeView`.
pub async fn create_top_level_account(
    new_account_id: AccountId,
    new_account_pk: PublicKey,
) -> anyhow::Result<Option<CallExecutionResult>> {
    let rt = crate::runtime::context::current().expect(MISSING_RUNTIME_ERROR);
    rt.create_top_level_account(new_account_id, new_account_pk)
        .await
}

pub async fn delete_account(
    account_id: AccountId,
    signer: &dyn Signer,
    beneficiary_id: AccountId,
) -> anyhow::Result<CallExecutionResult> {
    client::send_tx_and_retry(|| async {
        let (access_key, _, block_hash) =
            tool::access_key(account_id.clone(), signer.public_key()).await?;

        Ok(SignedTransaction::delete_account(
            access_key.nonce + 1,
            account_id.clone(),
            account_id.clone(),
            beneficiary_id.clone(),
            signer,
            block_hash,
        ))
    })
    .await
    .map(Into::into)
}

fn dev_generate() -> (AccountId, InMemorySigner) {
    let account_id = tool::random_account_id();
    let signer = InMemorySigner::from_seed(account_id.clone(), KeyType::ED25519, DEV_ACCOUNT_SEED);
    signer.write_to_file(&tool::credentials_filepath(account_id.clone()).unwrap());
    (account_id, signer)
}

pub async fn dev_create() -> anyhow::Result<(AccountId, InMemorySigner)> {
    let (account_id, signer) = dev_generate();
    let outcome = create_top_level_account(account_id.clone(), signer.public_key()).await?;
    dbg!(outcome);
    Ok((account_id, signer))
}

pub async fn dev_deploy(
    contract_file: impl AsRef<Path>,
) -> anyhow::Result<(AccountId, InMemorySigner)> {
    let (account_id, signer) = dev_generate();
    let outcome = crate::runtime::context::current()
        .expect(MISSING_RUNTIME_ERROR)
        .create_tla_and_deploy(
            account_id.clone(),
            signer.public_key(),
            &signer,
            contract_file,
        )
        .await?;
    dbg!(outcome);
    Ok((account_id, signer))
}
