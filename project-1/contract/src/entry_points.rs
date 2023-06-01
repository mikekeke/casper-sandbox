use alloc::{string::String, vec, vec::Vec};
use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter};

use crate::constants;

pub fn init() -> EntryPoint {
    EntryPoint::new(
        String::from(constants::init::ENDPOINT),
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn register() -> EntryPoint {
  EntryPoint::new(
      String::from(constants::registry::ENDPOINT),
      Vec::new(),
      CLType::Unit,
      EntryPointAccess::Public,
      EntryPointType::Contract,
  )
}

pub fn append() -> EntryPoint {
  EntryPoint::new(
      String::from(constants::append::ENDPOINT),
      vec![
        Parameter::new(constants::append::ARG, CLType::String)
      ],
      CLType::Unit,
      EntryPointAccess::Public,
      EntryPointType::Contract,
  )
}

pub fn mk_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init());
    entry_points.add_entry_point(register());
    entry_points.add_entry_point(append());

    entry_points
}
