use anyhow::Result;
use mockall::mock;
use std::io;

use resterrs::power_state::PowerState;
use resterrs::traits::power_state_change_handler::PowerStateChangeHandler;
use resterrs::traits::service_manager::{ServiceManager, ServiceStartCtx, ServiceStopCtx};

mock! {
    pub SystemdServiceManager {}

    impl ServiceManager for SystemdServiceManager {
        fn start(&self, ctx: ServiceStartCtx) -> io::Result<()>;
        fn stop(&self, ctx: ServiceStopCtx) -> io::Result<()>;
    }
}

mock! {
    pub PowerStateChangeHandler {}

    impl PowerStateChangeHandler for PowerStateChangeHandler {
        fn handle(&self, power_state: &PowerState) -> Result<()>;
    }
}
