pub (crate) mod contract {
  pub const PACKAGE_NAME: &str = "add-with-registry";
  pub const ACCESS_UREF: &str = "add-with-registry-uref";
  pub const KEY: &str = "add-with-registry-contract-key";
  pub const VERSION_KEY: &str = "add-with-registry-version";

}

pub (crate) mod init {
  pub const ENDPOINT: &str = "init-contract";
}

pub (crate) mod registry {
  pub const ENDPOINT: &str = "register-user-key";
  pub const DICT: &str = "registry-dict";
}

pub (crate) mod append {
  pub const ENDPOINT: &str = "append-chars";
  pub const ARG: &str = "what-to-append";
  pub const ACCUM_VALUE: &str = "accumulator-value";
}

pub (crate) mod test {
  pub const CONTRACT_WASM: &str = "contract.wasm";
}
