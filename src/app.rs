use crate::cli::Arguments;
use crate::config::Config;
use crate::handlers::{
    app_power_state_change_handler, commands_power_state_change_handler,
    systemd_power_state_change_handler,
};
use crate::logs::AppLogging;
use crate::power_state_change_manager::PowerStateChangeManager;
use crate::systemd::systemd_service_manager::SystemdServiceManager;
use crate::udev_power_monitor;
use anyhow::{Context, Result};
use std::sync::Arc;

pub struct App {
    cli_args: Arguments,
    config: Config,
}

impl App {
    pub fn new(cli_args: Arguments, config: Config) -> Self {
        Self { cli_args, config }
    }

    pub fn run(self) -> Result<()> {
        AppLogging::new(&self.cli_args.log_driver)
            .init()
            .context("Failed to initialize logging")?;

        let mut manager = PowerStateChangeManager::new();
        manager
            .add_handler(Arc::new(
                app_power_state_change_handler::AppPowerStateChangeHandler::new(
                    self.config.apps_to_stop,
                ),
            ))
            .add_handler(Arc::new(
                systemd_power_state_change_handler::SystemdPowerStateChangeHandler::new(
                    self.config.system_services_to_stop,
                    Box::new(SystemdServiceManager::system()),
                ),
            ))
            .add_handler(Arc::new(
                systemd_power_state_change_handler::SystemdPowerStateChangeHandler::new(
                    self.config.user_services_to_stop,
                    Box::new(SystemdServiceManager::user(self.config.username)),
                ),
            ))
            .add_handler(Arc::new(
                commands_power_state_change_handler::CommandsPowerStateChangeHandler::new(
                    self.config.commands_unplugged,
                    self.config.commands_plugged,
                ),
            ));

        let monitor = udev_power_monitor::UdevPowerMonitor::new(manager);
        monitor.start()
    }
}
