use std::io;

pub struct ServiceStartCtx {
    pub service_name: String,
}

pub struct ServiceStopCtx {
    pub service_name: String,
}

pub trait ServiceManager {
    fn start(&self, ctx: ServiceStartCtx) -> io::Result<()>;
    fn stop(&self, ctx: ServiceStopCtx) -> io::Result<()>;
}
