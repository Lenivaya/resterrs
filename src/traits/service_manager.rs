use std::io;

pub struct ServiceStartCtx {
    pub service_name: String,
}

pub struct ServiceStopCtx {
    pub service_name: String,
}

pub trait ServiceManager: Send + Sync {
    fn start(&self, ctx: ServiceStartCtx) -> io::Result<()>;
    fn stop(&self, ctx: ServiceStopCtx) -> io::Result<()>;
}

impl<T> From<T> for Box<dyn ServiceManager + Send + Sync>
where
    T: ServiceManager + 'static,
{
    fn from(manager: T) -> Self {
        Box::new(manager)
    }
}
