use crate::power_state::PowerState;
use crate::traits::power_state_change_handler::PowerStateChangeHandler;
use crate::traits::service_manager::{ServiceManager, ServiceStartCtx, ServiceStopCtx};
use anyhow::Result;

pub struct SystemdPowerStateChangeHandler {
    services: Vec<String>,
    service_manager: Box<dyn ServiceManager + Send + Sync>,
}

impl SystemdPowerStateChangeHandler {
    pub fn new(
        services: Vec<String>,
        service_manager: Box<dyn ServiceManager + Send + Sync>,
    ) -> Self {
        Self {
            services,
            service_manager,
        }
    }

    fn start_services(&self) {
        self.services
            .iter()
            .for_each(|service| self.start_service(service));
    }

    fn stop_services(&self) {
        self.services
            .iter()
            .for_each(|service| self.stop_service(service));
    }

    fn start_service(&self, service: &str) {
        tracing::info!("Starting service: {}", service);
        self.service_manager
            .start(ServiceStartCtx {
                service_name: service.to_string(),
            })
            .unwrap_or_else(|e| {
                tracing::error!("Could not start service: {}", e);
            })
    }

    fn stop_service(&self, service: &str) {
        tracing::info!("Stopping service: {}", service);
        self.service_manager
            .stop(ServiceStopCtx {
                service_name: service.to_string(),
            })
            .unwrap_or_else(|e| {
                tracing::error!("Could not stop service: {}", e);
            })
    }
}

impl PowerStateChangeHandler for SystemdPowerStateChangeHandler {
    fn handle(&self, power_state: &PowerState) -> Result<()> {
        match power_state {
            PowerState::Plugged => self.start_services(),
            PowerState::Unplugged => self.stop_services(),
        }

        Ok(())
    }
}
