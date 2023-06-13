extern crate alloc;
use alloc::string::{String, ToString};

use casper_event_standard::casper_types as es_types;
use casper_event_standard::{Event, Schemas};

#[derive(Event, Debug, PartialEq)]
pub struct SomeEvent {
    pub emitted_by: String,
}