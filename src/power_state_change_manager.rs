use crate::common::PowerState;
use crate::traits::power_state_change_handler::PowerStateChangeHandler;
use anyhow::Result;
use std::sync::Arc;
use std::thread;

pub struct PowerStateChangeManager {
    handlers: Vec<Arc<dyn PowerStateChangeHandler + Send + Sync>>,
}

impl Default for PowerStateChangeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerStateChangeManager {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(
        &mut self,
        handler: Arc<dyn PowerStateChangeHandler + Send + Sync>,
    ) -> &mut Self {
        self.handlers.push(handler);
        self
    }

    pub fn handle(&self, power_state: PowerState) -> Result<()> {
        for handler in self.handlers.iter() {
            let handler = Arc::clone(handler);
            let power_state = power_state.clone();
            thread::spawn(move || {
                if let Err(e) = handler.handle(&power_state) {
                    tracing::error!("Error handling power state change: {:?}", e);
                }
            });
        }

        Ok(())
    }
}
