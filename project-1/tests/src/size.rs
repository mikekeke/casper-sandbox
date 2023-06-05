use casper_types::Gas;

use crate::utility::misc;

#[test]
fn cost_regression() {
  let (_, builder) = misc::deploy_contract();
  let gas = builder.last_exec_gas_cost();
  let expected_gas: Gas = Gas::from(21310818500 as u64);
  assert_eq!(gas, expected_gas);
}