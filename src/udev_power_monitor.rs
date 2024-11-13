use crate::power_state::PowerState;
use crate::power_state_change_manager::PowerStateChangeManager;
use crate::power_state_tracker::PowerStateTracker;
use anyhow::{Context, Result};
use mio::{Events, Interest, Poll, Token};
use std::sync::Mutex;
use std::time::Duration;
use udev::MonitorBuilder;

const UDEV: Token = Token(0);

pub struct UdevPowerMonitor {
    power_state_change_manager: PowerStateChangeManager,
    pub power_state_tracker: Mutex<PowerStateTracker>,
}

impl UdevPowerMonitor {
    pub fn new(power_state_change_manager: PowerStateChangeManager) -> Self {
        UdevPowerMonitor {
            power_state_change_manager,
            power_state_tracker: Mutex::new(PowerStateTracker::new()),
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

        tracing::info!("Listening for power supply udev events...");

        loop {
            if let Err(e) = poll.poll(&mut events, Some(Duration::from_secs(1))) {
                tracing::error!("Failed to poll for events: {:?}", e);
                continue;
            };

            for event in events.iter() {
                if let UDEV = event.token() {
                    if let Some(udev_event) = socket.iter().next() {
                        if self.filter_event(&udev_event) {
                            self.handle_event(&udev_event);
                        }
                    }
                }
            }
        }
    }

    /// Filters out events that are not related to battery directly
    fn filter_event(&self, event: &udev::Event) -> bool {
        let sysname = event.sysname().to_str().unwrap_or("");

        sysname.starts_with("BAT")
            || sysname.starts_with("AC")
            || self.filter_event_properties(event)
    }

    fn filter_event_properties(&self, event: &udev::Event) -> bool {
        event
            .properties()
            .any(|p| self.filter_event_property(&p))
    }

    fn filter_event_property(&self, property: &udev::Entry) -> bool {
        (property.name() == "POWER_SUPPLY_TYPE" && property.value() == "Battery")
            || (property.name() == "POWER_SUPPLY_NAME" && property.value() == "AC")
    }

    fn handle_event(&self, event: &udev::Event) {
        let power_state = PowerState::from(event.attribute_value("online"));

        if self
            .power_state_tracker
            .lock()
            .unwrap()
            .should_handle(&power_state)
        {
            match power_state {
                PowerState::Plugged => tracing::info!("Power plugged in!"),
                PowerState::Unplugged => tracing::info!("Power unplugged!"),
            }

            if let Err(e) = self.power_state_change_manager.handle(power_state) {
                tracing::error!("Error handling power state change: {:?}", e);
            }
        }
    }
}
