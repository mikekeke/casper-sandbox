use core::convert::TryInto;

use casper_types::{ContractHash, URef, ApiError};
use casper_contract::{
  contract_api::{runtime, storage},
  unwrap_or_revert::UnwrapOrRevert,
};

use crate::constants;



pub (crate) fn get_contract_hash() -> ContractHash {
  let uref:URef = 
    runtime::get_key(constants::contract::KEY)
    .ok_or(ApiError::MissingKey)
    .unwrap_or_revert()
    .try_into()
    .unwrap_or_revert();

  storage::read(uref).unwrap_or_revert().unwrap_or_revert()
}