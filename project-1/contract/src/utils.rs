use core::convert::TryInto;

use alloc::string::{String, ToString};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_event_standard::Schemas;
use casper_types::{ApiError, ContractHash, URef};

use crate::constants;
use crate::events;

pub(crate) fn get_contract_hash() -> ContractHash {
    let uref: URef = runtime::get_key(constants::contract::KEY)
        .ok_or(ApiError::MissingKey)
        .unwrap_or_revert()
        .try_into()
        .unwrap_or_revert();

    storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}


pub(crate) fn caller_is_registered() -> (bool, String) {
    let account_hash = runtime::get_caller().to_string();
    let key = account_hash.as_str();
    let is_registered = storage::named_dictionary_get(constants::registry::DICT, key)
        .unwrap_or_revert()
        .unwrap_or(false);
    (is_registered, account_hash)
}

