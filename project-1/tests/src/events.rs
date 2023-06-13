use casper_engine_test_support::ExecuteRequestBuilder;
use casper_event_standard::casper_types::bytesrepr::{ToBytes, FromBytes};
use casper_types::{runtime_args, RuntimeArgs, bytesrepr::Bytes};

use crate::utility::{misc::{self, get_contract}, debug};
use contract::events::SomeEvent;

#[test]
fn event_emitted() {
    let (account_addr, mut builder) = misc::deploy_contract();

    let call_emit_event = ExecuteRequestBuilder::contract_call_by_hash(
        account_addr,
        misc::get_contract_hash(&builder, account_addr),
        "emit_event",
        runtime_args! {},
    )
    .build();
    builder.exec(call_emit_event).expect_success().commit();

    let contract = get_contract(&builder, account_addr);
    let seed_uref = *contract
        .named_keys()
        .get("__events")
        .expect("must have key")
        .as_uref()
        .expect("must convert to seed uref");
    println!("UREF: {:#?}", seed_uref);

    let stored_event_bytes: Bytes = builder
        .query_dictionary_item(None, seed_uref, "0")
        .expect("should have dictionary value")
        .as_cl_value()
        .expect("T should be CLValue")
        .to_owned()
        .into_t()
        .unwrap();

    let expected_event = SomeEvent {
        emitted_by: String::from("test"),
    };

    let stored_event = SomeEvent::from_bytes(&stored_event_bytes).unwrap().0;
    println!("EV: {:#?}", stored_event);
    assert_eq!(expected_event, stored_event);

}
