use anyhow::Result;
use clap::ValueEnum;
use tracing::subscriber::set_global_default;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{EnvFilter, Layer, Registry};

pub struct AppLogging<'a> {
    pub driver: &'a AppLoggingDriver,
}

impl<'a> AppLogging<'a> {
    pub fn new(driver: &'a AppLoggingDriver) -> Self {
        Self { driver }
    }

    pub fn init(self) -> Result<()> {
        let layer = self.driver.layer();
        let env_filter = EnvFilter::from_default_env();
        let subscriber = Registry::default().with(env_filter).with(layer);

        set_global_default(subscriber).expect("Failed to set global default subscriber");
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum AppLoggingDriver {
    #[default]
    Stdout,
    Stderr,
    Journald,
}

impl AppLoggingDriver {
    #[allow(clippy::type_repetition_in_bounds)]
    fn layer<S>(self) -> Option<Box<dyn Layer<S> + Send + Sync + 'static>>
    where
        S: tracing::Subscriber,
        for<'a> S: LookupSpan<'a>,
    {
        let fmt = tracing_subscriber::fmt::layer()
            .pretty()
            .with_thread_ids(true)
            .with_thread_names(true);

        match self {
            Self::Stdout => Some(Box::new(fmt.with_writer(std::io::stdout))),
            Self::Stderr => Some(Box::new(fmt.with_writer(std::io::stderr))),
            Self::Journald => Some(Box::new(tracing_journald::layer().ok()?)),
        }
    }
}
