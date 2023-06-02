use casper_engine_test_support::InMemoryWasmTestBuilder;
use casper_types::{bytesrepr::FromBytes, CLTyped, Key, account::AccountHash};

fn query_key<R>(builder: &InMemoryWasmTestBuilder, some_key: Key) -> R
where
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


