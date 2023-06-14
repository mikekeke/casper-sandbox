pub mod contract {
    pub const PACKAGE_NAME: &str = "add-with-registry-package";
    pub const ACCESS_UREF: &str = "add-with-registry-uref";
    pub const KEY: &str = "add-with-registry-contract-key";
    pub const VERSION_KEY: &str = "add-with-registry-version";
}

pub mod init {
    // endoint value should match with "nf_name" in  "pub extern "C" fn nf_name"
    pub const ENTRYPOINT: &str = "init";
}

pub mod registry {
    pub const ENTRYPOINT: &str = "register_user_key";
    pub const DICT: &str = "registry-dict";
}

pub mod append {
    pub const ENTRYPOINT: &str = "append_chars";
    pub const ARG: &str = "what-to-append";
    pub const ACCUM_VALUE: &str = "accumulator-value";
}

pub mod events {
    pub const SOME_EVENT_MSG: &str = "some_event_message";
}
