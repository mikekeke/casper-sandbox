use std::path::PathBuf;

use casper_engine_test_support::{ExecuteRequestBuilder, WasmTestBuilder};

use casper_execution_engine::{
    core::{
        engine_state::{self},
        execution,
    },
    storage::global_state::{CommitProvider, StateProvider},
};

use casper_types::{bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs};

use crate::utility::{constants, setup, debug};

#[test]
fn init_test() {
    let (account_addr, mut builder) = setup::setup_chain();

    let execute_request = ExecuteRequestBuilder::standard(
        account_addr,
        constants::test::CONTRACT_WASM,
        runtime_args! {},
    )
    .build();

    builder.exec(execute_request).commit().expect_success();

    debug::print_keys(&builder, account_addr);

    // let acc = builder.get_expected_account(account_addr);
    // let account_named_keys = acc.named_keys();
    // println!("Account named keys:\n{:#?}", account_named_keys);

    // let execute_request = ExecuteRequestBuilder::standard(
    //     account_addr,
    //     constants::test::CONTRACT_WASM,
    //     runtime_args! {},
    // )
    // .build();

    // builder.exec(execute_request).commit().expect_success();

    // let r = ACCESS_UREF;

    // make assertions
    // let result_of_query = builder
    //     .query(None, Key::Account(account_addr), &[KEY.to_string()])
    //     .expect("should be stored value.")
    //     .as_cl_value()
    //     .expect("should be cl value.")
    //     .clone()
    //     .into_t::<String>()
    //     .expect("should be string.");

    // println!("After init value: {}", result_of_query);

    // let binding = builder.get_expected_account(account_addr);

    // let account_named_keys = binding.named_keys();
    // println!("Account named keys:\n{:#?}", account_named_keys);

    // let contract_hash = account_named_keys
    //     .get("mut_str_contract")
    //     .expect("must have contract hash key as part of contract creation")
    //     .into_hash()
    //     .map(ContractHash::new)
    //     .expect("must get contract hash");
    // println!("Contract hash: {:#?}", contract_hash);

    // let contract = builder
    //     .get_contract(contract_hash)
    //     .expect("this contract should exist");
    // println!("Contract before init: {:#?}", contract);

    // let mut_val_key = contract
    //     .named_keys()
    //     .get("mut-val")
    //     .expect("Key for mutable value should exist");

    // let mut_val: String = query_key(&builder, *mut_val_key);
    // println!("Mut val before EP call: {:#?}", mut_val);

    // // call init
    // let call_init = ExecuteRequestBuilder::contract_call_by_hash(
    //     account_addr,
    //     contract_hash,
    //     "init",
    //     runtime_args! {},
    // )
    // .build();
    // builder.exec(call_init).expect_success().commit();

    // // call w dict
    // let call_cnt = ExecuteRequestBuilder::contract_call_by_hash(
    //     account_addr,
    //     contract_hash,
    //     "call_backed_by_dict",
    //     runtime_args! {},
    // )
    // .build();
    // builder.exec(call_cnt).expect_failure().commit();

    // let rr = builder.get_exec_result_owned(1);
    // println!("RES CNT: {:#?}", rr);

    // let contract = builder
    //     .get_contract(contract_hash)
    //     .expect("this contract should exist");
    // println!("Calls num before init: {:#?}", contract);

    // // call append to value
    // let call_ep = ExecuteRequestBuilder::contract_call_by_hash(
    //     account_addr,
    //     contract_hash,
    //     "append_to_value",
    //     runtime_args! {
    //         "what-to-add" => "integration-test-1"
    //     },
    // )
    // .build();
    // builder.exec(call_ep).expect_success().commit();

    // let mut_val: String = query_key(&builder, *mut_val_key);
    // println!("Mut val after EP call: {:#?}", mut_val);

    // // call w dict afet append
    // let call_cnt = ExecuteRequestBuilder::contract_call_by_hash(
    //     account_addr,
    //     contract_hash,
    //     "call_backed_by_dict",
    //     runtime_args! {},
    // )
    // .build();
    // builder.exec(call_cnt).expect_success().commit();
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

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
