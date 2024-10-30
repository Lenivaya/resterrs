use crate::common::PowerState;
use crate::traits::power_state_change_handler::PowerStateChangeHandler;
use anyhow::Result;
use std::{
    ffi::OsStr,
    sync::{Arc, Mutex},
};
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

        self.system
            .lock()
            .expect("Failed to lock system when fetching processes for stopping app")
            .processes()
            .iter()
            .filter(|(_, process)| {
                self.managed_apps
                    .iter()
                    .any(|app| self.should_stop_process(process, app))
            })
            .for_each(|(pid, process)| {
                if !process.kill() {
                    tracing::error!("Failed to stop app {:?} (PID: {:?}", process.name(), pid);
                }
            });
    }

    fn should_stop_process(&self, process: &Process, app: &str) -> bool {
        let process_name = process
            .name()
            .to_ascii_lowercase()
            .to_string_lossy()
            .to_ascii_lowercase();
        let process_cmd = process
            .cmd()
            .join(OsStr::new(" "))
            .to_string_lossy()
            .to_ascii_lowercase();

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
