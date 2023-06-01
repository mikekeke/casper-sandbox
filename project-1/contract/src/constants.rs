pub (crate) mod contract {
  pub const PACKAGE_NAME: &str = "add-with-registry-package";
  pub const ACCESS_UREF: &str = "add-with-registry-uref";
  pub const KEY: &str = "add-with-registry-contract-key";
  pub const VERSION_KEY: &str = "add-with-registry-version";

}

pub (crate) mod init {
  // endoint value should match with "nf_name" in  "pub extern "C" fn nf_name"
  pub const ENDPOINT: &str = "init";
}

pub (crate) mod registry {
  pub const ENDPOINT: &str = "register_user_key";
  pub const DICT: &str = "registry-dict";
}

pub (crate) mod append {
  pub const ENDPOINT: &str = "append_chars";
  pub const ARG: &str = "what-to-append";
  pub const ACCUM_VALUE: &str = "accumulator-value";
}
