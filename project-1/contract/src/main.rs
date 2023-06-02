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
    vec,
    vec::Vec,
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::FromBytes, contracts::NamedKeys, ApiError, CLType, CLValue, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Key, NamedKey, RuntimeArgs,
};
use entry_points::mk_entry_points;
// use constants;

mod constants;
mod entry_points;
mod utils;

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    AlredayDeployed = 0,
    AlreadyInitialized = 1,
    UserAlreadyRegistered = 2,
    UnregisteredTriedToAdd = 3,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn init() {
    // TODO: initilaization check
    if let Some(_) = runtime::get_key(constants::registry::DICT) {
        runtime::revert(Error::AlreadyInitialized)
    }

    // dictionary will be created in contract context
    storage::new_dictionary(constants::registry::DICT).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn append_chars() {
    // TODO
}

#[no_mangle]
pub extern "C" fn register_user_key() {
    let account_hash = runtime::get_caller().to_string();
    let key = account_hash.as_str();

    let is_registered = storage::named_dictionary_get(constants::registry::DICT, key)
        .unwrap_or_revert()
        .unwrap_or(false);

    if is_registered {
        runtime::revert(Error::UserAlreadyRegistered);
    }

    storage::named_dictionary_put(constants::registry::DICT, key, true);
}

fn isntall_contract() -> () {
    let mut contract_keys = NamedKeys::new();

    let new_empty_val = storage::new_uref("");
    contract_keys.insert(
        constants::append::ACCUM_VALUE.to_string(),
        new_empty_val.into(),
    );

    let (contract_hash, contract_version) = storage::new_locked_contract(
        mk_entry_points(),
        Some(contract_keys),
        Some(constants::contract::PACKAGE_NAME.to_string()),
        // access URef required for at least upgrading the contract,
        // w/o it upgrade is not possible
        Some(constants::contract::ACCESS_UREF.to_string()),
    );

    runtime::put_key(constants::contract::KEY, contract_hash.into());
    runtime::put_key(
        constants::contract::VERSION_KEY,
        storage::new_uref(contract_version).into(),
    );

    runtime::call_contract(contract_hash, constants::init::ENDPOINT, RuntimeArgs::new())
}

#[no_mangle]
pub extern "C" fn call_backed_by_dict() {
    // // runtime::revert(Error::CntNotOne);

    // let caller = runtime::get_caller(); // todo get caller from ARGs
    // let registry = *runtime::get_key(constants::REGISTRY_DICT)
    //     .unwrap_or_revert_with(Error::RegistryNotFound)
    //     .as_uref()
    //     .unwrap_or_revert();

    // let caller_key = caller.to_string();
    // let cnt = storage::dictionary_get::<u64>(registry, &caller_key.as_str())
    //     .unwrap_or_revert()
    //     .unwrap_or(0);
    // if cnt != 1 {
    //     runtime::revert(Error::CntNotOne)
    // };
    // // ! looks like reutrn only works with `call_contract(...)`
    // // ! called from another contract
    // // runtime::ret(CLValue::from_t(cnt).unwrap_or_revert())
}

// #[no_mangle]
// pub extern "C" fn append_to_value() {
//     let mut_val_key = runtime::get_key(MUT_VAL);
//     if mut_val_key.is_none() {
//         runtime::revert(Error::KeyNotFound);
//     }

//     let what_to_add: String = runtime::get_named_arg(constants::ADD_ARG_NAME);
//     let mut current_val: String = storage::read_from_key(mut_val_key.unwrap_or_revert())
//         .unwrap_or_revert_with(ApiError::Read)
//         .unwrap_or_revert_with(ApiError::ValueNotFound);
//     if current_val.is_empty() {
//         current_val.push_str(&what_to_add);
//     } else {
//         current_val.push_str(";");
//         current_val.push_str(&what_to_add);
//     }

//     let key_uref = mut_val_key
//         .unwrap_or_revert_with(ApiError::MissingKey)
//         .into_uref()
//         .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
//     // storage::write(key_uref, new_val);
//     storage::write(key_uref, current_val);
//     registry_caller();
// }

// fn registry_caller() {
//     let caller = runtime::get_caller();

//     let registry = *runtime::get_key(constants::REGISTRY_DICT)
//         .unwrap_or_revert_with(Error::RegistryNotFound)
//         .as_uref()
//         .unwrap_or_revert();

//     let caller_key = caller.to_string();

//     match storage::dictionary_get::<u64>(registry, &caller_key.as_str()).unwrap_or_revert() {
//         None => storage::dictionary_put(registry, &caller_key, 1u64),
//         Some(x) => storage::dictionary_put(registry, &caller_key, x + 1u64),
//     }
// }

// }

#[no_mangle]
pub extern "C" fn call() {
    if let Some(_) = runtime::get_key(constants::contract::ACCESS_UREF) {
        runtime::revert(Error::AlredayDeployed)
    }
    isntall_contract();
}
