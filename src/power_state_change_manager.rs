use crate::common;
use crate::traits::power_state_change_handler::PowerStateChangeHandler;
use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PowerStateChangeManager {
    handlers: Vec<Rc<RefCell<Box<dyn PowerStateChangeHandler>>>>,
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

    pub fn add_handler(&mut self, handler: Box<dyn PowerStateChangeHandler>) -> &mut Self {
        self.handlers.push(Rc::new(RefCell::new(handler)));
        self
    }
}

impl PowerStateChangeHandler for PowerStateChangeManager {
    fn handle(&mut self, power_state: &common::PowerState) -> Result<()> {
        for handler in &self.handlers {
            handler
                .borrow_mut()
                .handle(power_state)
                .expect("Failed to handle power state change");
        }

        Ok(())
    }
}
