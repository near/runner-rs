# Changelog

## [Unreleased]

## [0.17.0](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.16.0...near-workspaces-v0.17.0) - 2024-12-26

### Added

- [**breaking**] `worker.dev_create_account`, `worker.dev_deploy` create subaccounts of root account instead of tla (sandbox, testnet) (#369)

## [0.16.0](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.15.0...near-workspaces-v0.16.0) - 2024-12-18

### Fixed

- include fix from https://github.com/near/cargo-near/pull/266 (#395)

### Other

- [**breaking**] updates near-* dependencies to 0.28 release (#392)

## [0.15.0](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.14.1...near-workspaces-v0.15.0) - 2024-11-18

### Other

- [**breaking**] updates near-* dependencies to 0.27 release ([#387](https://github.com/near/near-workspaces-rs/pull/387))
- [**breaking**] reexport `cargo-near-build` for configurable build if needed ([#386](https://github.com/near/near-workspaces-rs/pull/386))
- make cargo-near-build optional in manifest ([#383](https://github.com/near/near-workspaces-rs/pull/383))

## [0.14.1](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.14.0...near-workspaces-v0.14.1) - 2024-10-18

### Fixed

- TransactionStatus::status should not be waiting for Final ([#379](https://github.com/near/near-workspaces-rs/pull/379))

### Other

- fix clippy 1.82 (doc preambles) ([#381](https://github.com/near/near-workspaces-rs/pull/381))
- bump cargo-near-build to 0.2.0 ([#380](https://github.com/near/near-workspaces-rs/pull/380))

## [0.14.0](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.13.0...near-workspaces-v0.14.0) - 2024-09-12

### Other

- [**breaking**] Updated near-* dependencies to 0.26 release ([#377](https://github.com/near/near-workspaces-rs/pull/377))

## [0.13.0](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.12.0...near-workspaces-v0.13.0) - 2024-09-10

### Other

- [**breaking**] updates near-* dependencies to 0.25 ([#375](https://github.com/near/near-workspaces-rs/pull/375))
- replace `cargo-near` with `cargo-near-build` ([#373](https://github.com/near/near-workspaces-rs/pull/373))

## [0.12.0](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.11.1...near-workspaces-v0.12.0) - 2024-08-15

### Other
- [**breaking**] upgraded near deps to 0.24 ([#370](https://github.com/near/near-workspaces-rs/pull/370))

## [0.11.1](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.11.0...near-workspaces-v0.11.1) - 2024-08-07

### Fixed
- Gracefully handle account creation request errors from a faucet service [testnet-only] ([#366](https://github.com/near/near-workspaces-rs/pull/366))

## [0.11.0](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.10.1...near-workspaces-v0.11.0) - 2024-07-05

### Other
- Upgraded deps ([#363](https://github.com/near/near-workspaces-rs/pull/363))

## [0.10.1](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.10.0...near-workspaces-v0.10.1) - 2024-06-26

### Added
- Exposes status RPC API method ([#308](https://github.com/near/near-workspaces-rs/pull/308))
- Fixed `dev_deploy` to use registrar account to create top-level accounts (nearcore 1.37.0+ forbidden to create top-level accounts by non-registrar accounts) ([#360](https://github.com/near/near-workspaces-rs/pull/360))
- Re-exported near-abi-client ([#303](https://github.com/near/near-workspaces-rs/pull/303))

### Other
- a way to get account with more than 100 near on sandbox ([#351](https://github.com/near/near-workspaces-rs/pull/351))
- Updated cargo-near to "0.5.2" ([#347](https://github.com/near/near-workspaces-rs/pull/347))

## [0.10.0](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.9.0...near-workspaces-v0.10.0) - 2024-01-25

### Other
- Impl Clone on result Value ([#345](https://github.com/near/near-workspaces-rs/pull/345))
- Upgraded NEAR crates to 0.20.0 release ([#346](https://github.com/near/near-workspaces-rs/pull/346))
- dependecy bumps ([#338](https://github.com/near/near-workspaces-rs/pull/338))
- cleanup internals ([#329](https://github.com/near/near-workspaces-rs/pull/329))
- use stable sandbox by default ([#335](https://github.com/near/near-workspaces-rs/pull/335))
- [**breaking**] Remove `interop_sdk` feature from defaults ([#339](https://github.com/near/near-workspaces-rs/pull/339))
- fix typos ([#340](https://github.com/near/near-workspaces-rs/pull/340))

## [0.9.0](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.8.0...near-workspaces-v0.9.0) - 2023-10-30

### Added
- Require Send + Sync for T in Worker<T> to allow near-workspaces usage in multithreading async runtimes  ([#328](https://github.com/near/near-workspaces-rs/pull/328))
- Added API for measuring gas ([#284](https://github.com/near/near-workspaces-rs/pull/284))

### Fixed
- Fixed await during acquired lock in rpc::client::fetch_tx_nonce ([#334](https://github.com/near/near-workspaces-rs/pull/334))

### Other
- [**breaking**] Migrate to use NearToken instead of raw u128 for balance variables to ensure strict typing ([#333](https://github.com/near/near-workspaces-rs/pull/333))
- Restrict GasHook type to require RefUnwindSafe and UnwindSafe to avoid breaking release ([#323](https://github.com/near/near-workspaces-rs/pull/323)) ([#326](https://github.com/near/near-workspaces-rs/pull/326))
- remove unwraps ([#321](https://github.com/near/near-workspaces-rs/pull/321))

## [0.8.0](https://github.com/near/near-workspaces-rs/compare/near-workspaces-v0.7.0...near-workspaces-v0.8.0) - 2023-10-04

- [**breaking**] renamed crate to near-workspaces to avoid confusion with Cargo workspaces; imports should now use `near_workspaces` instead of just `workspaces` ([#318](https://github.com/near/near-workspaces-rs/pull/318))
- [Upgraded to Rust Stable Toolchain](https://github.com/near/near-workspaces-rs/commit/8d93197d06aee2a426b6da99e270ce1658c2cd4f). Deprecates requirement of only using rustc-1.69 and lower.

### Added

- [Import a couple functions over from near_crypto for PublicKey](https://github.com/near/workspaces-rs/pull/265)
  - Impl `Ord`, `PartialOrd`, `Hash`, `BorshSerialize`, `BorshDeserialize`, `Display`, and `FromStr` for `PublicKey`
    - NOTE: Borsh bytes format is the same as near-sdk, where it is in the form of [bytes_len, key_type, key_data..]
  - Added `PublicKey::{empty, len, key_data}`
  - Impl `Display` for `SecretKey`.
  - more docs were added to both `SecretKey` and `PublicKey`.
  - Impl `Display`, `FromStr`, `TryFrom<u8>` for `KeyType`.
- [Added `TryFrom<near_sdk::PublicKey>` for `workspaces::PublicKey`](https://github.com/near/workspaces-rs/pull/267)
  - Added `KeyType::len` and `PublicKey::try_from_bytes`
- [Added experimental apis from near-sdk-rs](https://github.com/near/near-workspaces-rs/pull/285), available under the **experimental** flag.
  - Methods added are: EXPERIMENTAL_changes_in_block, EXPERIMENTAL_changes, EXPERIMENTAL_genesis_config, EXPERIMENTAL_protocol_config, EXPERIMENTAL_receipt, EXPERIMENTAL_tx_status, EXPERIMENTAL_validators_ordered
- [Added Worker::patch to patch account, keys, code, and state in a generic builder](https://github.com/near/near-workspaces-rs/pull/291)
  - Added `Worker::patch` and `PatchTransaction` that provide builders for patching accounts, keys, code, and state.
  - Added `AccountDetails` and `AccountDetailsPatch` which hold the state of the patch.
- Allow to select a specific version of near-sandbox ([#311](https://github.com/near/near-workspaces-rs/pull/311))
- Enable support for RPCs that require API keys and support for custom networks ([#306](https://github.com/near/near-workspaces-rs/pull/306))
- expose more `Block` and `Chunk` fields ([#243](https://github.com/near/near-workspaces-rs/pull/243))
- support manually supplied validator key ([#274](https://github.com/near/near-workspaces-rs/pull/274))

### Changed

- [`Transaction::transact_async` no longer has a lifetime parameter to make it easier to use](https://github.com/near/workspaces-rs/pull/249)
- [Improved error message on calling a json on a void function](https://github.com/near/near-workspaces-rs/pull/286)
- [Removed serde-arbitrary-precision feature in examples](https://github.com/near/near-workspaces-rs/pull/287)

### Fixed

- improve error msg on calling `json` on void function ([#286](https://github.com/near/near-workspaces-rs/pull/286))
- fix typos ([#280](https://github.com/near/near-workspaces-rs/pull/280))
- Run `neard` on `localhost` instead of `0.0.0.0` to prevent firewall popups on MacOS ([#277](https://github.com/near/near-workspaces-rs/pull/277))
- storing credentials ([#258](https://github.com/near/near-workspaces-rs/pull/258))
- Make call consistent with worker::view ([#245](https://github.com/near/near-workspaces-rs/pull/245))

### Other

- drop async-process in favor of tokio ([#316](https://github.com/near/near-workspaces-rs/pull/316))
- switch to `near-gas` crate for Gas where possible ([#305](https://github.com/near/near-workspaces-rs/pull/305))
- Improved fast_forward docs ([#299](https://github.com/near/near-workspaces-rs/pull/299))
- Added test for delete_account ([#289](https://github.com/near/near-workspaces-rs/pull/289))
- Added a test for transfer_near ([#290](https://github.com/near/near-workspaces-rs/pull/290))
- using url return type ([#297](https://github.com/near/near-workspaces-rs/pull/297))
- dependencies and removed unused deps ([#292](https://github.com/near/near-workspaces-rs/pull/292))
- upgrade to stable toolchain ([#293](https://github.com/near/near-workspaces-rs/pull/293))
- Updated near deps to 0.17 ([#283](https://github.com/near/near-workspaces-rs/pull/283))
- Use cargo-near to build project ([#275](https://github.com/near/near-workspaces-rs/pull/275))
- Added network builder for mainnet, testnet, betanet ([#221](https://github.com/near/near-workspaces-rs/pull/221))
- bump borsh version and other deps ([#271](https://github.com/near/near-workspaces-rs/pull/271))
- bump sandbox to 0.6.2 ([#270](https://github.com/near/near-workspaces-rs/pull/270))
- Import some functions over from near_crypto for PublicKey ([#265](https://github.com/near/near-workspaces-rs/pull/265))
- Added destination account-id for `import_contract` call ([#260](https://github.com/near/near-workspaces-rs/pull/260))
- Fix port collision ([#257](https://github.com/near/near-workspaces-rs/pull/257))
- Removed the lifetime in transact_async ([#249](https://github.com/near/near-workspaces-rs/pull/249))
- configure sandbox ([#251](https://github.com/near/near-workspaces-rs/pull/251))

## [0.7.0]

### Added

- [`view_*` asynchronous builders have been added which provides being able to query from a specific block hash or block height](https://github.com/near/workspaces-rs/pull/218)
- [`{CallTransaction, Transaction}::transact_async` for performing transactions without directly having to wait for it complete it on chain](https://github.com/near/workspaces-rs/pull/222)
- [`view_chunk` added for querying into chunk related info on the network.](https://github.com/near/workspaces-rs/pull/234)
  - Adds `Chunk` and `ChunkHeader` type to reference specific chunk info.
- [`Error::{simple, message, custom}` are now public and usable for custom errors](https://github.com/near/workspaces-rs/pull/224)

### Changed

- [Apart of the changes from adding `view_*` async builders, we have a couple breaking changes to the `view_*` functions](https://github.com/near/workspaces-rs/pull/218):
  - `{Account, Contract, Worker}::view_state` moved `prefix` parameter into builder. i.e.
    ```
    worker.view_state("account_id", Some(prefix)).await?;
    // is now
    worker.view_state("account_id")
        .prefix(prefix)
        .await?;
    // if prefix was `None`, then simply delete the None argument.
    ```
  - `view` function changed to be a builder, and no longer take in `args` as a parameter. It instead has been moved to the builder side.
  - Changed `Worker::view_latest_block` to `Worker::view_block` as the default behavior is equivalent.
  - `operations::Function` type no longer takes a lifetime parameter.
  - `operations::CallTransaction` type takes one less lifetime parameter.
- [`Worker::call` signature changed to be more in line with `view_*` async builders. It will now return a builder like `{Account, Contract}::call`](https://github.com/near/workspaces-rs/pull/245)
  - This `call` no longer accepts `Contract` since that was not as accessible. Instead a `InMemorySigner` is now required to sign transactions (which can be retrieved from `{Account, Contract}::signer` or `InMemorySigner::{from_secret_key, from_file}`).
  - `{Account, Contract}::signer` now exposed.

### Fixed

- [Changed the docs to reflect proper size of of rate limits on near.org RPC](https://github.com/near/workspaces-rs/pull/219)
- [Cached nonces now are per account-id and public-key instead of just public-key](https://github.com/near/workspaces-rs/pull/231)
  - this didn't matter if only one KeyPair was being used per account, but could be problematic when there were multiple KeyPairs per account utilizing the same nonces.
- [Error message context wasn't being exposed properly by sandbox, so this fixed it](https://github.com/near/workspaces-rs/pull/236)

## [0.6.1]

### Fixed

- Fixed query variant error when supply invalid function name or arguments: https://github.com/near/workspaces-rs/pull/239

## [0.6.0]

### Added

- `Account::view` API exposed: https://github.com/near/workspaces-rs/pull/202

### Changed

- Unstable `compile_project` uses new the workspaces errors: https://github.com/near/workspaces-rs/pull/204
- `ValueOrReceiptId::Value(String)` changed to `ValueOrReceiptId::Value(Value)`: https://github.com/near/workspaces-rs/pull/208
  - `Value` type offers convenient APIs like `raw_bytes`, `json`, and `borsh` like one would find from a `ExecutionFinalResult`.
- internal dependencies like near-jsonrpc-client upgraded to 0.4.0 from 0.4.0-beta: https://github.com/near/workspaces-rs/pull/210
  - Note, the RNG for `SecretKey::{from_random, from_seed}(KeyType::SECP256K1, ...)` has been changed as well, and will produce different keys than before.

### Fixed

- `docs.rs` now shows `unstable` feature flag: https://github.com/near/workspaces-rs/pull/198
- No longer orphaning sandbox processes on early termination of tests: https://github.com/near/workspaces-rs/pull/205
- Fixed sandbox colliding installs: https://github.com/near/workspaces-rs/pull/211
- sandbox no longer spamming stats logs: https://github.com/near/workspaces-rs/pull/213

## [0.5.0]

### Added

- Error handling with opaque `workspaces::error::Error` type: https://github.com/near/workspaces-rs/pull/149
- Require `#[must_use]` on the Execution value returned by `transact()`: https://github.com/near/workspaces-rs/pull/150
  - Added `ExecutionFinalResult`, `ExecutionResult`, `ExecutionSuccess` and `ExecutionFailure` types
  - Added `into_result()` to easily handle `#[must_use] ExecutionFinalResult`
  - Added `unwrap()` to not care about Err variant in `ExecutionResult`s

### Changed

- Renamed CallExecution\* types: https://github.com/near/workspaces-rs/pull/150
  - Renamed ` CallExecution`` to  `Execution`
  - Renamed `CallExecutionDetails` to `ExecutionFinalResult`
- `args_json` and `args_borsh` no longer return `Result`s and are deferred till later when `transact()`ed: https://github.com/near/workspaces-rs/pull/149
- API changes from removing `worker` parameter from function calls: https://github.com/near/workspaces-rs/pull/181
  - `Account::from_file` function signature change, requiring a `&worker` to be passed in.
  - `workspaces::prelude::*` import no longer necessary, where we no longer able to import `workspaces::prelude::DevAccountDeployer` directly.

### Removed

- Removed impls from exection result: https://github.com/near/workspaces-rs/pull/150
  - Removed `impl<T> From<CallExecution<T>> for Result<T>`
  - Removed `impl From<FinalExecutionOutcomeView> for CallExecutionDetails`
- No longer require `worker` to be passed in for each transaction: https://github.com/near/workspaces-rs/pull/181

### Fixed

- Gas estimation issue resolved with latest sandbox node (Aug 29, 2022): https://github.com/near/workspaces-rs/pull/188
- Fixed parallel tests, where calling into the same contract would require waiting on a previous call: https://github.com/near/workspaces-rs/pull/173

## [0.4.1] - 2022-08-16

### Added

- Derive `Eq` on `AccountDetails` type: https://github.com/near/workspaces-rs/pull/177/files

### Fixed

- Fix macOS non-deterministic overflow error when starting up sandbox: https://github.com/near/workspaces-rs/pull/179

## [0.4.0] - 2022-07-20

### Added

- Mac M1 Support: https://github.com/near/workspaces-rs/pull/169
- Added `Account::secret_key` to grab the account's secret key: https://github.com/near/workspaces-rs/pull/144
- `Debug`/`Clone` impls for `Account`/`Contract`, and `Debug` for `Worker`: https://github.com/near/workspaces-rs/pull/167
- `ExecutionOutcome::tokens_burnt` is now available: https://github.com/near/workspaces-rs/pull/168

### Fixed

- internally no longer creating a new RPC client per call: https://github.com/near/workspaces-rs/pull/154
- upped near dependencies to fix transitive vulnerabilities: https://github.com/near/workspaces-rs/pull/169

### Changed

- Default sandbox version is now using commit hash master/13a66dda709a4148f6395636914dca2a55df1390 (July 18, 2022): https://github.com/near/workspaces-rs/pull/169

## [0.3.1] - 2022-06-20

### Added

- Raw bytes API similar to `json`/`borsh` calls: https://github.com/near/workspaces-rs/pull/133/files
- Expose `types` module and added `SecretKey` creation: https://github.com/near/workspaces-rs/pull/139

### Fixed

- If sandbox gets started multiple times, short circuit it early on: https://github.com/near/workspaces-rs/pull/135
- Fix short timeouts on connecting to RPC for macos with custom env variable to specify timeout if needed: https://github.com/near/workspaces-rs/pull/143

## [0.3.0] - 2022-05-10

### Added

- Added betanet support https://github.com/near/workspaces-rs/pull/116

### Changed

- Updated default sandbox version to `97c0410de519ecaca369aaee26f0ca5eb9e7de06` commit of nearcore to include 1.26 protocol changes https://github.com/near/workspaces-rs/pull/134

- Exposed `CallExecutionDetails::raw_bytes` API: https://github.com/near/workspaces-rs/pull/133

## [0.2.1] - 2022-04-12

### Added

- Added more docs to top level or exposed types/functions: https://github.com/near/workspaces-rs/pull/115

### Fixed

- Fix `docs.rs` builds failing on sandbox install: https://github.com/near/workspaces-rs/pull/115

## [0.2.0] - 2022-04-05

### Added

- Time-traveling - the ability to go forwards in block height within tests. This allows to test time specific data changing within contracts: https://github.com/near/workspaces-rs/pull/73
- Credentials created from account/contract creation are now allowed to be stored and specified by users. https://github.com/near/workspaces-rs/pull/98
- [Unstable] Allow users to compile contract projects within tests without having to manually go through this step. https://github.com/near/workspaces-rs/pull/77
- Batch transactions or transactions with multiple actions are now possible. https://github.com/near/workspaces-rs/pull/72
- Sandbox node (nearcore binary) logs are now suppressed and can be re-enabled if desired. https://github.com/near/workspaces-rs/pull/85
- Results now expose logs, receipts, and transaction outcome values. https://github.com/near/workspaces-rs/pull/70
- Convenience methods `Worker::view_code`, `Worker::view_latest_block`, `Worker::view_account`, `Account::view_account`, `Contract::view_account`, `Contract::view_code` now available. https://github.com/near/workspaces-rs/pull/82
- Improve error handling. If a transaction fails, this error will now be apart of the `Result` return initially. https://github.com/near/workspaces-rs/pull/83
- Added `tracing` logging to internal code and examples. https://github.com/near/workspaces-rs/pull/55 and https://github.com/near/workspaces-rs/pull/75
- Convenient `CallExecutionDetails::{is_success, is_failure}` for testing outcomes of transactions. https://github.com/near/workspaces-rs/pull/58
- Added `mainnet_archival` and `testnet_archival`, where `ref-finance` example now uses `mainnet_archival`. https://github.com/near/workspaces-rs/pull/57 and https://github.com/near/workspaces-rs/pull/94

### Changed

- key type for `patch_state` now a slice and no longer require `StoreKey`. https://github.com/near/workspaces-rs/pull/109
- Reorganized imports internally for better maintainability. https://github.com/near/workspaces-rs/pull/102
- No longer running into non-deterministic query failures if RPC isn't available, but this is a breaking API. All `workspaces::{sandbox, testnet, mainnet}` now require `.await?` at the end. https://github.com/near/workspaces-rs/pull/99
- TLA trait no longer apart of all networks -- only dev-networks (sandbox, testnet). https://github.com/near/workspaces-rs/pull/101
- Retry times have now been shorten and should take a maximum of 1 second. https://github.com/near/workspaces-rs/pull/92
- doc builds on [docs.rs](https://docs.rs) has now been fixed. https://github.com/near/workspaces-rs/pull/90
- `patch_state` now takes in slices. https://github.com/near/workspaces-rs/pull/80 and https://github.com/near/workspaces-rs/pull/79
- Make `access_key` call do optimistic queries which led to better retry times. https://github.com/near/workspaces-rs/pull/60
- Functions no longer take in owned but referenced `AccountId`s now. https://github.com/near/workspaces-rs/pull/52

### Removed

- Empty JSON array is no longer a valid default argument supplied to transactions. Recommended to supply empty `{}` in the case of JSON if all function arguments in the contract are optional types. https://github.com/near/workspaces-rs/pull/84

## [0.1.1] - 2021-01-24

### Changed

- Fix race condition when installing sandbox and running multiples tests at the same time. https://github.com/near/workspaces-rs/pull/46

[unreleased]: https://github.com/near/workspaces-rs/compare/0.7.0...HEAD
[0.7.0]: https://github.com/near/workspaces-rs/compare/0.6.1...0.7.0
[0.6.1]: https://github.com/near/workspaces-rs/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/near/workspaces-rs/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/near/workspaces-rs/compare/0.4.1...0.5.0
[0.4.1]: https://github.com/near/workspaces-rs/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/near/workspaces-rs/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/near/workspaces-rs/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/near/workspaces-rs/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/near/workspaces-rs/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/near/workspaces-rs/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/near/workspaces-rs/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/near/workspaces-rs/releases/tag/0.1.0
