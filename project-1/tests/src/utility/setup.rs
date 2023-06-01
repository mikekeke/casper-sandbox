

use std::path::PathBuf;


use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_CHAINSPEC_REGISTRY, DEFAULT_GENESIS_CONFIG,
    DEFAULT_GENESIS_CONFIG_HASH,
};

use casper_execution_engine::{
    core::{
        engine_state::{self, run_genesis_request::RunGenesisRequest, GenesisAccount},
        execution,
    },
    storage::global_state::{CommitProvider, StateProvider},
};

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractHash, Key, Motes,
    PublicKey, RuntimeArgs, SecretKey, U512,
};

const CONTRACT_WASM: &str = "contract.wasm";

pub fn setup_chain() -> (
    AccountHash,
    WasmTestBuilder<casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState>,
) {
    const MY_ACCOUNT: [u8; 32] = [7u8; 32];
    // Create keypair.
    let secret_key = SecretKey::ed25519_from_bytes(MY_ACCOUNT).unwrap();
    let public_key = PublicKey::from(&secret_key);

    // Create an AccountHash from a public key.
    let account_addr = AccountHash::from(&public_key);
    // Create a GenesisAccount.
    let account = GenesisAccount::account(
        public_key,
        Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
        None,
    );

    let mut genesis_config = DEFAULT_GENESIS_CONFIG.clone();
    genesis_config.ee_config_mut().push_account(account);

    let run_genesis_request = RunGenesisRequest::new(
        *DEFAULT_GENESIS_CONFIG_HASH,
        genesis_config.protocol_version(),
        genesis_config.take_ee_config(),
        DEFAULT_CHAINSPEC_REGISTRY.clone(),
    );
    // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.  Paths
    // relative to the current working dir (e.g. 'wasm/contract.wasm') can also be used, as can
    // absolute paths.

    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&run_genesis_request).commit();
    (account_addr, builder)
}