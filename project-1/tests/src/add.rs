use casper_engine_test_support::ExecuteRequestBuilder;
use casper_types::{runtime_args, RuntimeArgs};

use crate::utility::{assert, constants, misc};

#[test]
fn unregistered_cant_add() {
    let (account_addr, mut builder) = misc::deploy_contract();
    let call_register = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        constants::append::ENDPOINT,
        runtime_args! {},
    )
    .build();
    builder.exec(call_register).expect_failure().commit();
    let err = builder.get_error().expect("should be error");
    assert::assert_expected_error(
        err,
        3,
        "should throw an error corresponding to double registration",
    );
}
