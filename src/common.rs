use std::ffi::OsStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PowerState {
    Plugged,
    Unplugged,
}

impl From<&OsStr> for PowerState {
    fn from(value: &OsStr) -> Self {
        match value.to_str() {
            Some("0") => PowerState::Unplugged,
            _ => PowerState::Plugged,
        }
    }
}

impl From<Option<&OsStr>> for PowerState {
    fn from(value: Option<&OsStr>) -> Self {
        match value {
            Some(value) => PowerState::from(value),
            _ => PowerState::Plugged,
        }
    }
}
