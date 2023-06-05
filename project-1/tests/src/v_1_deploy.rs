use casper_engine_test_support::ExecuteRequestBuilder;

use casper_types::{runtime_args, RuntimeArgs};

use crate::utility::{assert, constants, debug, misc};

#[test]
fn deploy() {
    let (account_addr, mut builder) = misc::deploy_contract();
    debug::print_keys(&builder, account_addr);

    let contract = misc::get_contract(&builder, account_addr);
    let contract_keys = contract.named_keys();

    contract_keys
        .get(constants::append::ACCUM_VALUE)
        .expect("Accum value should exist after contract initialization");

    contract_keys
        .get(constants::registry::DICT)
        .expect("Registry dict should exist after contract initialization");

    println!("{:#?}", contract_keys);

    let cost = builder.exec_costs(0);
    println!("Cost: {:#?}", cost);
}

#[test]
fn can_not_deploy_second_time() {
    let (account_addr, mut builder) = misc::deploy_contract();
    let execute_request = ExecuteRequestBuilder::standard(
        account_addr,
        "contract.wasm",
        runtime_args! {},
    )
    .build();

    builder.exec(execute_request).commit().expect_failure();
}

#[test]
fn can_not_init_second_time() {
    let (account_addr, mut builder) = misc::deploy_contract();
    let execute_request = ExecuteRequestBuilder::standard(
        account_addr,
        "contract.wasm",
        runtime_args! {},
    )
    .build();

    // deployment logic calls `init` inside `call`
    builder.exec(execute_request).commit().expect_failure();

    let call_init = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::init::ENDPOINT,
        runtime_args! {},
    )
    .build();
    builder.exec(call_init).expect_failure().commit();

    let err = builder.get_error().expect("should be error");
    assert::assert_expected_error(
        err,
        1,
        "should throw an error corresponding to double initialization",
    );
    // print!("Err: {:#?}", err);
}

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
