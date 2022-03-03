use std::collections::HashMap;

use async_trait::async_trait;
use near_primitives::types::{Balance, StoreKey};

use crate::network::{
    Account, AllowDevAccountCreation, Block, CallExecution, CallExecutionDetails, Contract,
    NetworkClient, NetworkInfo, StatePatcher, TopLevelAccountCreator, ViewResultDetails,
};
use crate::network::{Info, Sandbox};
use crate::rpc::client::{Client, DEFAULT_CALL_DEPOSIT, DEFAULT_CALL_FN_GAS};
use crate::rpc::patch::ImportContractBuilder;
use crate::types::{AccountId, Gas, InMemorySigner, SecretKey};
use crate::worker::Worker;
use crate::{AccountDetails, Network};

impl<T> Clone for Worker<T> {
    fn clone(&self) -> Self {
        Self {
            workspace: self.workspace.clone(),
        }
    }
}

impl<T> AllowDevAccountCreation for Worker<T> where T: AllowDevAccountCreation {}

#[async_trait]
impl<T> TopLevelAccountCreator for Worker<T>
where
    T: TopLevelAccountCreator + Send + Sync,
{
    async fn create_tla(
        &self,
        id: AccountId,
        sk: SecretKey,
    ) -> anyhow::Result<CallExecution<Account>> {
        self.workspace.create_tla(id, sk).await
    }

    async fn create_tla_and_deploy(
        &self,
        id: AccountId,
        sk: SecretKey,
        wasm: &[u8],
    ) -> anyhow::Result<CallExecution<Contract>> {
        self.workspace.create_tla_and_deploy(id, sk, wasm).await
    }
}

impl<T> NetworkInfo for Worker<T>
where
    T: NetworkInfo,
{
    fn info(&self) -> &Info {
        self.workspace.info()
    }
}

#[async_trait]
impl<T> StatePatcher for Worker<T>
where
    T: StatePatcher + Send + Sync,
{
    async fn patch_state(
        &self,
        contract_id: &AccountId,
        key: &[u8],
        value: &[u8],
    ) -> anyhow::Result<()> {
        self.workspace.patch_state(contract_id, key, value).await
    }

    fn import_contract<'a, 'b>(
        &'b self,
        id: &AccountId,
        worker: &'a Worker<impl Network>,
    ) -> ImportContractBuilder<'a, 'b> {
        self.workspace.import_contract(id, worker)
    }
}

impl<T> Worker<T>
where
    T: NetworkClient,
{
    pub(crate) fn client(&self) -> &Client {
        self.workspace.client()
    }

    /// Call into a contract's change function.
    pub async fn call(
        &self,
        contract: &Contract,
        function: &str,
        args: Vec<u8>,
        gas: Option<Gas>,
        deposit: Option<Balance>,
    ) -> anyhow::Result<CallExecutionDetails> {
        self.client()
            .call(
                contract.signer(),
                contract.id(),
                function.into(),
                args,
                gas.unwrap_or(DEFAULT_CALL_FN_GAS),
                deposit.unwrap_or(DEFAULT_CALL_DEPOSIT),
            )
            .await
            .and_then(CallExecutionDetails::from_outcome)
    }

    /// Call into a contract's view function.
    pub async fn view(
        &self,
        contract_id: &AccountId,
        function: &str,
        args: Vec<u8>,
    ) -> anyhow::Result<ViewResultDetails> {
        self.client()
            .view(contract_id.clone(), function.into(), args)
            .await
    }

    /// View the WASM code bytes of a contract on the network.
    pub async fn view_code(&self, contract_id: &AccountId) -> anyhow::Result<Vec<u8>> {
        let code_view = self.client().view_code(contract_id.clone(), None).await?;
        Ok(code_view.code)
    }

    /// View the state of a account/contract on the network. This will return the internal
    /// state of the account in the form of a map of key-value pairs; where STATE contains
    /// info on a contract's internal data.
    pub async fn view_state(
        &self,
        contract_id: &AccountId,
        prefix: Option<StoreKey>,
    ) -> anyhow::Result<HashMap<String, Vec<u8>>> {
        self.client().view_state(contract_id.clone(), prefix).await
    }

    /// View the latest block from the network
    pub async fn view_latest_block(&self) -> anyhow::Result<Block> {
        self.client().view_block(None).await.map(Into::into)
    }

    /// Transfer tokens from one account to another. The signer is the account
    /// that will be used to to send from.
    pub async fn transfer_near(
        &self,
        signer: &InMemorySigner,
        receiver_id: &AccountId,
        amount_yocto: Balance,
    ) -> anyhow::Result<CallExecutionDetails> {
        self.client()
            .transfer_near(signer, receiver_id, amount_yocto)
            .await
            .and_then(CallExecutionDetails::from_outcome)
    }

    /// Deletes an account from the network. The beneficiary will receive the balance
    /// of the account deleted.
    pub async fn delete_account(
        &self,
        account_id: &AccountId,
        signer: &InMemorySigner,
        beneficiary_id: &AccountId,
    ) -> anyhow::Result<CallExecutionDetails> {
        self.client()
            .delete_account(signer, account_id, beneficiary_id)
            .await
            .and_then(CallExecutionDetails::from_outcome)
    }

    /// View account details of a specific account on the network.
    pub async fn view_account(&self, account_id: &AccountId) -> anyhow::Result<AccountDetails> {
        self.client()
            .view_account(account_id.clone(), None)
            .await
            .map(Into::into)
    }
}

impl Worker<Sandbox> {
    pub fn root_account(&self) -> Account {
        let account_id = self.info().root_id.clone();
        let signer = self.workspace.root_signer();
        Account::new(account_id, signer)
    }
}
