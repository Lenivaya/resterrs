use crate::power_state::PowerState;
use anyhow::Result;

pub trait PowerStateChangeHandler: Send + Sync {
    fn handle(&self, power_state: &PowerState) -> Result<()>;
}
