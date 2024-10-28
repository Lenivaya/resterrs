use crate::common::PowerState;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PowerStateTracker {
    last_state: Option<PowerState>,
}

impl PowerStateTracker {
    pub fn new() -> Self {
        Self { last_state: None }
    }

    pub fn should_handle(&mut self, new_state: &PowerState) -> bool {
        let should_handle = match &self.last_state {
            None => true,                    // First event, which is always handled
            Some(last) => last != new_state, // Handle only if state changed
        };

        if should_handle {
            self.last_state = Some(new_state.clone());
        }

        should_handle
    }
}
