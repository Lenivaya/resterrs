use crate::common::PowerState;
use crate::traits::power_state_change_handler::PowerStateChangeHandler;
use anyhow::{Context, Result};
use run_script::run_script;

#[derive(Debug, Clone, Default)]
pub struct CommandsPowerStateChangeHandler {
    pub commands_unplugged: Option<Vec<String>>,
    pub commands_plugged: Option<Vec<String>>,
}

impl CommandsPowerStateChangeHandler {
    pub fn new(
        commands_unplugged: Option<Vec<String>>,
        commands_plugged: Option<Vec<String>>,
    ) -> Self {
        Self {
            commands_unplugged,
            commands_plugged,
        }
    }

    pub fn run_commands(&self, commands: &Option<Vec<String>>) -> Result<()> {
        if let Some(commands) = commands {
            for command in commands {
                tracing::info!("Running command: {}", command);
                if let Err(e) = run_script!(command)
                    .with_context(|| format!("Failed to run command: {}", command))
                {
                    tracing::error!("{}", e);
                }
            }
        }
        Ok(())
    }
}

impl PowerStateChangeHandler for CommandsPowerStateChangeHandler {
    fn handle(&self, power_state: &PowerState) -> Result<()> {
        match power_state {
            PowerState::Plugged => self.run_commands(&self.commands_plugged),
            PowerState::Unplugged => self.run_commands(&self.commands_unplugged),
        }
    }
}
