use casper_contract::contract_api::storage;
use casper_engine_test_support::ExecuteRequestBuilder;
use casper_types::{runtime_args, RuntimeArgs};

use crate::utility::{assert, constants, debug, misc};

#[test]
fn user_registration() {
    let (account_addr, mut builder) = misc::deploy_contract();
    let call_register = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::registry::ENDPOINT,
        runtime_args! {},
    )
    .build();
    builder.exec(call_register).expect_success().commit();

    let key = account_addr.to_string();
    let is_registered: bool = misc::named_dictionary_get(
        &builder,
        account_addr,
        constants::registry::DICT,
        key.as_str(),
    );
    assert_eq!(is_registered, true);
}

#[test]
fn can_not_register_twice() {
    let (account_addr, mut builder) = misc::deploy_contract();
    let call_register = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::registry::ENDPOINT,
        runtime_args! {},
    )
    .build();
    builder.exec(call_register).expect_success().commit();

    let call_register = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::registry::ENDPOINT,
        runtime_args! {},
    )
    .build();
    builder.exec(call_register).expect_failure().commit();
    let err = builder.get_error().expect("should be error");
    assert::assert_expected_error(
        err,
        2,
        "should throw an error corresponding to double registration",
    );
}
