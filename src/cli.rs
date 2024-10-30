use crate::logs::AppLoggingDriver;
use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[command(name = "rester.rs", author, about, version)]
pub struct Arguments {
    /// Path to the configuration file
    /// (omit to use default xdg-compliant)
    #[arg(
        name = "config-file",
        value_hint = ValueHint::FilePath,
        required = false,
        short
    )]
    pub config_file: Option<String>,

    /// Where logs must go
    #[arg(
        long,
        value_enum,
        value_name("LOG_DRIVER"),
        default_value_t = AppLoggingDriver::Stdout
    )]
    pub log_driver: AppLoggingDriver,
}
