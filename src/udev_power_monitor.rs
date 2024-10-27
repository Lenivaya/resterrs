use crate::common::PowerState;
use crate::power_state_change_manager::PowerStateChangeManager;
use anyhow::{Context, Result};
use log::info;
use mio::{Events, Interest, Poll, Token};
use std::time::Duration;
use udev::MonitorBuilder;

const UDEV: Token = Token(0);

pub struct UdevPowerMonitor {
    power_state_change_manager: PowerStateChangeManager,
}

impl UdevPowerMonitor {
    pub fn new(power_state_change_manager: PowerStateChangeManager) -> Self {
        UdevPowerMonitor {
            power_state_change_manager,
        }
    }

    pub fn start(&self) -> Result<()> {
        let mut socket = MonitorBuilder::new()
            .context("Failed to create udev monitor builder")?
            .match_subsystem_devtype("power_supply", "power_supply")
            .context("Failed to match subsystem and devtype")?
            .listen()
            .context("Failed to listen for udev events")?;

        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(1);

        poll.registry()
            .register(&mut socket, UDEV, Interest::READABLE)?;

        info!("Listening for power supply udev events...");

        loop {
            poll.poll(&mut events, Some(Duration::from_secs(1)))?;

            for event in events.iter() {
                if let UDEV = event.token() {
                    if let Some(udev_event) = socket.iter().next() {
                        self.handle_event(&udev_event);
                    }
                }
            }
        }
    }

    fn handle_event(&self, event: &udev::Event) {
        let power_state = PowerState::from(event.attribute_value("online"));
        match power_state {
            PowerState::Plugged => info!("Power plugged in!"),
            PowerState::Unplugged => info!("Power unplugged!"),
        }
        if let Err(e) = self.power_state_change_manager.handle(power_state) {
            eprintln!("Error handling power state change: {:?}", e);
        }
    }
}
