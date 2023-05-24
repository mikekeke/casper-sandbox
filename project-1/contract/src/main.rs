#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{
    fmt::format,
    string::{String, ToString},
    vec::Vec,
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, ApiError, CLType, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Key, NamedKey, CLValue,
};
use contract_consts::MUT_VAL;

mod contract_consts;

const MY_KEY_NAME: &str = "my-key-name";
const RUNTIME_ARG_NAME: &str = "message";

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    KeyAlreadyExists = 0,
    KeyMismatch = 1,
    KeyNotFound = 404,
    RegistryNotFound = 13,
    CntNotOne=111,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn call_backed_by_dict() {
    // runtime::revert(Error::CntNotOne);

    let caller = runtime::get_caller(); // todo get caller from ARGs
    let registry = *runtime::get_key(contract_consts::REGISTRY_DICT)
        .unwrap_or_revert_with(Error::RegistryNotFound)
        .as_uref()
        .unwrap_or_revert();

    let caller_key = caller.to_string();
    let cnt = storage::dictionary_get::<u64>(registry, &caller_key.as_str())
        .unwrap_or_revert()
        .unwrap_or(0);
    if cnt != 1 {
        runtime::revert(Error::CntNotOne)
    };
    // ! looks like reutrn only works with `call_contract(...)`
    // ! called from another contract
    // runtime::ret(CLValue::from_t(cnt).unwrap_or_revert()) 
}

#[no_mangle]
pub extern "C" fn init() {
    storage::new_dictionary(contract_consts::REGISTRY_DICT).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn append_to_value() {
    let mut_val_key = runtime::get_key(MUT_VAL);
    if mut_val_key.is_none() {
        runtime::revert(Error::KeyNotFound);
    }

    let what_to_add: String = runtime::get_named_arg(contract_consts::ADD_ARG_NAME);
    let mut current_val: String = storage::read_from_key(mut_val_key.unwrap_or_revert())
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);
    if current_val.is_empty() {
        current_val.push_str(&what_to_add);
    } else {
        current_val.push_str(";");
        current_val.push_str(&what_to_add);
    }

    let key_uref = mut_val_key
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    // storage::write(key_uref, new_val);
    storage::write(key_uref, current_val);
    registry_caller();
}

fn registry_caller() {
    let caller = runtime::get_caller();

    let registry = *runtime::get_key(contract_consts::REGISTRY_DICT)
        .unwrap_or_revert_with(Error::RegistryNotFound)
        .as_uref()
        .unwrap_or_revert();

    let caller_key = caller.to_string();

    match storage::dictionary_get::<u64>(registry, &caller_key.as_str()).unwrap_or_revert() {
        None => storage::dictionary_put(registry, &caller_key, 1u64),
        Some(x) => storage::dictionary_put(registry, &caller_key, x + 1u64),
    }
}

fn install_entrypoints(entry_points: &mut EntryPoints) {
    entry_points.add_entry_point(EntryPoint::new(
        contract_consts::EP_INIT,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        contract_consts::EP_APPEND,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        contract_consts::EP_CALL_W_DICT,
        Vec::new(),
        // CLType::Option<Box<CLType>>, //! ???
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
}

fn make_contract() {
    let empty = storage::new_uref("empty");

    let mut contract_keys = NamedKeys::new();
    contract_keys.insert(String::from(contract_consts::MUT_VAL), empty.into());

    // let dict_uref = storage::new_dictionary(contract_consts::REGISTRY_DICT).unwrap_or_revert();
    // contract_keys.insert(
    //     String::from(contract_consts::REGISTRY_DICT),
    //     dict_uref.into(),
    // );

    let mut entry_points = EntryPoints::new();
    install_entrypoints(&mut entry_points);

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(contract_keys),
        Some(contract_consts::CONTRACT_PACKAGE_NAME.to_string()),
        // access URef required for at least upgrading the contract,
        // w/o it upgrade is not possible
        Some(contract_consts::CONTRACT_ACCESS_UREF.to_string()),
    );

    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(contract_consts::CONTRACT_VERSION_KEY, version_uref.into());

    // Create a named key for the contract hash.
    runtime::put_key(contract_consts::CONTRACT_KEY, contract_hash.into());
}

#[no_mangle]
pub extern "C" fn call() {
    // The key shouldn't already exist in the named keys.
    let missing_key = runtime::get_key(MY_KEY_NAME);
    if missing_key.is_some() {
        runtime::revert(Error::KeyAlreadyExists);
    }

    // This contract expects a single runtime argument to be provided.  The arg is named "message"
    // and will be of type `String`.
    let value: String = runtime::get_named_arg(RUNTIME_ARG_NAME);

    // Store this value under a new unforgeable reference a.k.a `URef`.
    let value_ref = storage::new_uref(value);
    // storage::add(value_ref, "lol");

    // Store the new `URef` as a named key with a name of `MY_KEY_NAME`.
    let key = Key::URef(value_ref);
    runtime::put_key(MY_KEY_NAME, key);

    // The key should now be able to be retrieved.  Note that if `get_key()` returns `None`, then
    // `unwrap_or_revert()` will exit the process, returning `ApiError::None`.
    let retrieved_key = runtime::get_key(MY_KEY_NAME).unwrap_or_revert();

    if retrieved_key != key {
        runtime::revert(Error::KeyMismatch);
    }
    make_contract();
}
