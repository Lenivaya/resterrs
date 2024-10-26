use std::thread;
use std::time::Duration;

use crate::common::PowerState;
use crate::power_state_change_manager::PowerStateChangeManager;
use crate::traits::power_state_change_handler::PowerStateChangeHandler;
use anyhow::{Context, Result};
use log::info;
use udev::MonitorBuilder;

pub struct UdevPowerMonitor<'a> {
    power_state_change_manager: &'a mut PowerStateChangeManager,
}

impl<'a> UdevPowerMonitor<'a> {
    pub fn new(power_state_change_manager: &'a mut PowerStateChangeManager) -> Self {
        UdevPowerMonitor {
            power_state_change_manager,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        let mut monitor = MonitorBuilder::new()
            .context("Failed to create udev monitor builder")?
            .match_subsystem_devtype("power_supply", "power_supply")
            .context("Failed to match subsystem and devtype")?
            .listen()
            .context("Failed to listen for udev events")?;

        info!("Listening for power supply udev events...");

        loop {
            let events: Vec<_> = monitor.iter().collect();
            if let Some(event) = events.last() {
                self.handle_event(event);
            }

            thread::sleep(Duration::from_secs(1));
        }
    }

    pub fn handle_event(&mut self, event: &udev::Event) {
        let power_state = PowerState::from(event.attribute_value("online"));
        match power_state {
            PowerState::Plugged => info!("Power plugged in!"),
            PowerState::Unplugged => info!("Power unplugged!"),
        }
        self.power_state_change_manager
            .handle(&power_state)
            .expect("Failed to handle power state change");
    }
}
