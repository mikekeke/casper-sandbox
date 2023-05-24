#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_INITIAL_BALANCE,
        DEFAULT_CHAINSPEC_REGISTRY, DEFAULT_GENESIS_CONFIG, DEFAULT_GENESIS_CONFIG_HASH,
    };
    use casper_execution_engine::{
        core::{
            engine_state::{self, run_genesis_request::RunGenesisRequest, GenesisAccount},
            execution,
        },
        storage::global_state::{CommitProvider, StateProvider},
    };
    use casper_types::{
        account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractHash, Key,
        Motes, PublicKey, RuntimeArgs, SecretKey, U512,
    };

    const MY_ACCOUNT: [u8; 32] = [7u8; 32];
    // Define `KEY` constant to match that in the contract.
    const KEY: &str = "my-key-name";
    const VALUE: &str = "hello world";
    const RUNTIME_ARG_NAME: &str = "message";
    const CONTRACT_WASM: &str = "contract.wasm";

    fn prepare_env() {}

    #[test]
    fn should_store_hello_world() {
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

        // prepare assertions.
        let result_of_query = builder.query(
            None,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            &[KEY.to_string()],
        );
        assert!(result_of_query.is_err());

        let execute_request = ExecuteRequestBuilder::standard(
            account_addr,
            CONTRACT_WASM,
            runtime_args! {RUNTIME_ARG_NAME => VALUE},
        )
        .build();

        builder.exec(execute_request).commit().expect_success();

        // make assertions
        let result_of_query = builder
            .query(None, Key::Account(account_addr), &[KEY.to_string()])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<String>()
            .expect("should be string.");

        println!("After init value: {}", result_of_query);

        let binding = builder.get_expected_account(account_addr);

        let account_named_keys = binding.named_keys();
        println!("Account named keys:\n{:#?}", account_named_keys);

        let contract_hash = account_named_keys
            .get("mut_str_contract")
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");
        println!("Contract hash: {:#?}", contract_hash);

        let contract = builder
            .get_contract(contract_hash)
            .expect("this contract should exist");
        println!("Contract before init: {:#?}", contract);

        let mut_val_key = contract
            .named_keys()
            .get("mut-val")
            .expect("Key for mutable value should exist");

        let mut_val: String = query_key(&builder, *mut_val_key);
        println!("Mut val before EP call: {:#?}", mut_val);

        // call init
        let call_init = ExecuteRequestBuilder::contract_call_by_hash(
            account_addr,
            contract_hash,
            "init",
            runtime_args! {},
        )
        .build();
        builder.exec(call_init).expect_success().commit();

        // call w dict
        let call_cnt = ExecuteRequestBuilder::contract_call_by_hash(
            account_addr,
            contract_hash,
            "call_backed_by_dict",
            runtime_args! {},
        )
        .build();
        builder.exec(call_cnt).expect_failure().commit();

        let rr = builder.get_exec_result_owned(1);
        println!("RES CNT: {:#?}", rr);

        let contract = builder
            .get_contract(contract_hash)
            .expect("this contract should exist");
        println!("Calls num before init: {:#?}", contract);

        // call append to value
        let call_ep = ExecuteRequestBuilder::contract_call_by_hash(
            account_addr,
            contract_hash,
            "append_to_value",
            runtime_args! {
                "what-to-add" => "integration-test-1"
            },
        )
        .build();
        builder.exec(call_ep).expect_success().commit();

        let mut_val: String = query_key(&builder, *mut_val_key);
        println!("Mut val after EP call: {:#?}", mut_val);

        // call w dict afet append
        let call_cnt = ExecuteRequestBuilder::contract_call_by_hash(
            account_addr,
            contract_hash,
            "call_backed_by_dict",
            runtime_args! {},
        )
        .build();
        builder.exec(call_cnt).expect_success().commit();
    }

    fn query_key<S, R>(builder: &WasmTestBuilder<S>, some_key: Key) -> R
    where
        S: StateProvider + CommitProvider,
        engine_state::Error: From<S::Error>,
        S::Error: Into<execution::Error>,
        R: CLTyped + FromBytes,
    {
        builder
            .query(None, some_key, &[])
            .expect(format!("Should be able to query {:#?}", some_key).as_str())
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<R>()
            .expect(format!("Could not into {}", std::any::type_name::<R>()).as_str())
    }

    // #[test]
    // fn should_error_on_missing_runtime_arg() {
    //     let secret_key = SecretKey::ed25519_from_bytes(MY_ACCOUNT).unwrap();
    //     let public_key = PublicKey::from(&secret_key);
    //     let account_addr = AccountHash::from(&public_key);

    //     let session_code = PathBuf::from(CONTRACT_WASM);
    //     let session_args = RuntimeArgs::new();

    //     let deploy_item = DeployItemBuilder::new()
    //         .with_empty_payment_bytes(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
    //         .with_authorization_keys(&[account_addr])
    //         .with_address(*DEFAULT_ACCOUNT_ADDR)
    //         .with_session_code(session_code, session_args)
    //         .build();

    //     let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy_item).build();

    //     let mut builder = InMemoryWasmTestBuilder::default();
    //     builder
    //         .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
    //         .exec(execute_request)
    //         .commit()
    //         .expect_failure();
    // }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
