use crate::common::PowerState::{Plugged, Unplugged};
use std::ffi::OsStr;

pub enum PowerState {
    Plugged,
    Unplugged,
}

impl From<&OsStr> for PowerState {
    fn from(value: &OsStr) -> Self {
        match value.to_str() {
            Some("0") => Unplugged,
            _ => Plugged,
        }
    }
}

impl From<Option<&OsStr>> for PowerState {
    fn from(value: Option<&OsStr>) -> Self {
        match value {
            Some(value) => PowerState::from(value),
            _ => Plugged,
        }
    }
}
