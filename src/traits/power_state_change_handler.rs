use crate::common::PowerState;
use anyhow::Result;

pub trait PowerStateChangeHandler {
    fn handle(&mut self, power_state: &PowerState) -> Result<()>;
}
