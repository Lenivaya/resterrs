use crate::common::PowerState;
use crate::traits::power_state_change_handler::PowerStateChangeHandler;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use sysinfo::{Process, ProcessRefreshKind, RefreshKind, System, UpdateKind};

pub struct AppPowerStateChangeHandler {
    managed_apps: Vec<String>,
    system: Arc<Mutex<System>>,
}

impl AppPowerStateChangeHandler {
    pub fn new(apps_to_manage: Vec<String>) -> Self {
        Self {
            managed_apps: apps_to_manage
                .into_iter()
                .map(|app| app.to_lowercase())
                .collect(),
            system: Arc::new(Mutex::new(System::new_with_specifics(
                RefreshKind::new().with_processes(
                    ProcessRefreshKind::new()
                        .with_cmd(UpdateKind::Always)
                        .with_exe(UpdateKind::Always),
                ),
            ))),
        }
    }

    fn stop_apps(&self) {
        self.system.lock().unwrap().refresh_all();

        for app in &self.managed_apps {
            log::info!("Stopping app: {}", app);
            self.system
                .lock()
                .expect("Failed to lock system when fetching processes for stopping app")
                .processes()
                .iter()
                .filter(|(_, process)| self.should_stop_process(process, app))
                .for_each(|(pid, process)| {
                    if !process.kill() {
                        log::error!("Failed to stop app {:?} (PID: {:?}", process.name(), pid);
                    }
                });
        }
    }

    fn should_stop_process(&self, process: &Process, app: &str) -> bool {
        let process_name = process
            .name()
            .to_ascii_lowercase()
            .to_string_lossy()
            .to_ascii_lowercase();
        let process_cmd = process
            .cmd()
            .first()
            .map(|cmd| cmd.to_string_lossy().to_ascii_lowercase())
            .unwrap_or_default();

        process_name.contains(app) || process_cmd.contains(app)
    }
}

impl PowerStateChangeHandler for AppPowerStateChangeHandler {
    fn handle(&self, power_state: &PowerState) -> Result<()> {
        match power_state {
            PowerState::Unplugged => self.stop_apps(),
            PowerState::Plugged => {} // Do nothing when plugged in
        }

        Ok(())
    }
}
