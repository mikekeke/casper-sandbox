extern crate alloc;
use alloc::string::{String};


use casper_event_standard::{Event};

#[derive(Event, Debug, PartialEq)]
pub struct SomeEvent {
    pub message: String,
}