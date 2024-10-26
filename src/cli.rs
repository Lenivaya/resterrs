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
}
