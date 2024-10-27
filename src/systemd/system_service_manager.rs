use crate::traits::service_manager::{ServiceManager, ServiceStartCtx, ServiceStopCtx};
use log::info;
use std::io;
use std::process::{Command, Stdio};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SystemdServiceManager {
    pub user: Option<String>,
}

impl SystemdServiceManager {
    pub fn system() -> Self {
        Self::default()
    }

    pub fn user(user: String) -> Self {
        Self { user: Some(user) }
    }

    pub fn into_system(self) -> Self {
        Self { user: None }
    }

    pub fn into_user(self, user: String) -> Self {
        Self { user: Some(user) }
    }

    pub fn is_user(&self) -> bool {
        self.user.is_some()
    }
}

impl ServiceManager for SystemdServiceManager {
    fn start(&self, ctx: ServiceStartCtx) -> io::Result<()> {
        systemctl("start", &ctx.service_name, self.user.as_ref())
    }

    fn stop(&self, ctx: ServiceStopCtx) -> io::Result<()> {
        systemctl("stop", &ctx.service_name, self.user.as_ref())
    }
}

static SYSTEMCTL: &str = "systemctl";

fn systemctl(cmd: &str, label: &str, user: Option<&String>) -> io::Result<()> {
    let output = {
        let mut command = Command::new(SYSTEMCTL);

        command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if user.is_some() {
            let user = user.unwrap();
            command.arg("--user").arg(format!("--machine={}@", user));
        };

        command.arg(cmd).arg(label);

        info!("Running systemctl command: {:?}", command);

        command.output()?
    };

    if output.status.success() {
        Ok(())
    } else {
        let msg = String::from_utf8(output.stderr)
            .ok()
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                String::from_utf8(output.stdout)
                    .ok()
                    .filter(|s| !s.trim().is_empty())
            })
            .unwrap_or_else(|| format!("Failed to {cmd} for {label}"));

        Err(io::Error::new(io::ErrorKind::Other, msg))
    }
}
