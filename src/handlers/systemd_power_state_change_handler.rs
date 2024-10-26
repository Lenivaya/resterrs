use crate::common::PowerState;
use crate::traits::power_state_change_handler::PowerStateChangeHandler;
use crate::traits::service_manager::{ServiceManager, ServiceStartCtx, ServiceStopCtx};
use anyhow::Result;
// use service_manager::ServiceManager;
// use service_manager::*;

pub struct SystemdPowerStateChangeHandler {
    services: Vec<String>,
    service_manager: Box<dyn ServiceManager>,
}

impl SystemdPowerStateChangeHandler {
    pub fn new(services: Vec<String>, service_manager: Box<dyn ServiceManager>) -> Self {
        Self {
            services,
            service_manager,
        }
    }

    pub fn handle_udev_event(&self, power_state: &PowerState) {
        match power_state {
            PowerState::Plugged => self.start_services(),
            PowerState::Unplugged => self.stop_services(),
        }
    }

    pub fn start_services(&self) {
        for service in &self.services {
            log::info!("Starting service: {}", service);
            self.service_manager
                .start(ServiceStartCtx {
                    service_name: service.clone(),
                })
                .unwrap_or_else(|e| {
                    log::error!("Could not start service: {}", e);
                })
            // self.service_manager
            //     .start(ServiceStartCtx {
            //         label: ServiceLabel {
            //             application: service.clone(),
            //             qualifier: None,
            //             organization: None,
            //         },
            //     })
            //     .unwrap_or_else(|e| {
            //         log::error!("Could not start service: {}", e);
            //     })
        }
    }

    pub fn stop_services(&self) {
        for service in &self.services {
            log::info!("Stopping service: {}", service);
            self.service_manager
                .stop(ServiceStopCtx {
                    service_name: service.clone(),
                })
                .unwrap_or_else(|e| {
                    log::error!("Could not stop service: {}", e);
                })
            // self.service_manager
            //     .stop(ServiceStopCtx {
            //         label: ServiceLabel {
            //             application: service.clone(),
            //             qualifier: None,
            //             organization: None,
            //         },
            //     })
            //     .unwrap_or_else(|e| {
            //         log::error!("Could not stop service: {}", e);
            //     })
        }
    }
}

impl PowerStateChangeHandler for SystemdPowerStateChangeHandler {
    fn handle(&mut self, power_state: &PowerState) -> Result<()> {
        match power_state {
            PowerState::Plugged => self.start_services(),
            PowerState::Unplugged => self.stop_services(),
        }

        Ok(())
    }
}
