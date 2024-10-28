use anyhow::Result;
use clap::Parser;
use cli::Arguments;
use resterrs::config::Config;
use resterrs::handlers::{
    app_power_state_change_handler, commands_power_state_change_handler,
    systemd_power_state_change_handler,
};
use resterrs::power_state_change_manager::PowerStateChangeManager;
use resterrs::systemd::systemd_service_manager::SystemdServiceManager;
use resterrs::{cli, udev_power_monitor};
use std::sync::Arc;

fn main() -> Result<()> {
    env_logger::init();
    let args = Arguments::parse();
    let config = Config::new(args);

    let mut manager = PowerStateChangeManager::new();
    manager
        .add_handler(Arc::new(
            app_power_state_change_handler::AppPowerStateChangeHandler::new(config.apps_to_stop),
        ))
        .add_handler(Arc::new(
            systemd_power_state_change_handler::SystemdPowerStateChangeHandler::new(
                config.system_services_to_stop,
                Box::new(SystemdServiceManager::system()),
            ),
        ))
        .add_handler(Arc::new(
            systemd_power_state_change_handler::SystemdPowerStateChangeHandler::new(
                config.user_services_to_stop,
                Box::new(SystemdServiceManager::user(config.username)),
            ),
        ))
        .add_handler(Arc::new(
            commands_power_state_change_handler::CommandsPowerStateChangeHandler::new(
                config.commands_unplugged,
                config.commands_plugged,
            ),
        ));

    let monitor = udev_power_monitor::UdevPowerMonitor::new(manager);
    monitor.start()
}
