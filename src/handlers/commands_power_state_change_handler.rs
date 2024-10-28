use crate::common::PowerState;
use crate::traits::power_state_change_handler::PowerStateChangeHandler;
use anyhow::Result;
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
                log::info!("Running command: {}", command);
                run_script!(command).expect(format!("Could not run command: {}", command).as_str());
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
