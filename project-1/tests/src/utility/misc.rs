use std::path::PathBuf;

use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder,
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
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Contract, ContractHash,
    Motes, PublicKey, RuntimeArgs, SecretKey, U512,
};

use crate::utility::constants;

pub(crate) fn setup_chain() -> (AccountHash, InMemoryWasmTestBuilder) {
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

pub(crate) fn get_contract_hash(
    builder: &InMemoryWasmTestBuilder,
    account_addr: AccountHash,
) -> ContractHash {
    let account = builder.get_expected_account(account_addr);
    let account_named_keys = account.named_keys();
    account_named_keys
        .get(constants::contract::KEY)
        .expect("must have contract hash key as part of contract creation")
        .into_hash()
        .map(ContractHash::new)
        .expect("must get contract hash")
}

pub(crate) fn get_contract(
    builder: &InMemoryWasmTestBuilder,
    account_addr: AccountHash,
) -> Contract {
    builder
        .get_contract(get_contract_hash(builder, account_addr))
        .expect("this contract should exist")
}

pub(crate) fn deploy_contract() -> (casper_types::account::AccountHash, InMemoryWasmTestBuilder) {
    let (account_addr, mut builder) = setup_chain();

    let execute_request = ExecuteRequestBuilder::standard(
        account_addr,
        constants::test::CONTRACT_WASM,
        runtime_args! {},
    )
    .build();

    builder.exec(execute_request).commit().expect_success();

    (account_addr, builder)
}

pub fn named_dictionary_get<R>(
    // pub fn named_dictionary_get(
    builder: &InMemoryWasmTestBuilder,
    account_hash: AccountHash,
    dict_name: &str,
    some_key: &str,
    // ) -> ()
) -> R
where
    R: CLTyped + FromBytes,
{
    let contract = get_contract(builder, account_hash);
    let dict_seed_uref = *contract
        .named_keys()
        .get_key_value(dict_name)
        .expect(format!("should return key-value pair for key `{}`", dict_name).as_str())
        .1
        .as_uref()
        .expect("should be URef");

    builder
        .query_dictionary_item(None, dict_seed_uref, some_key)
        .expect("should have dictionary value")
        .as_cl_value()
        .expect("T should be CLValue")
        .to_owned()
        .into_t()
        .unwrap()
}
