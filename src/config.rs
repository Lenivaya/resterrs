use std::path::PathBuf;
use std::{fs, sync::LazyLock};

use crate::cli::Arguments;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub static CONFIG_FOLDER: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = dirs::config_dir().expect("Could not find config directory");
    path.extend([APP_NAME]);
    path
});
pub static CONFIG_FILE: LazyLock<PathBuf> = LazyLock::new(|| CONFIG_FOLDER.join("config.toml"));

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    #[serde(default = "Vec::new")]
    /// System services which will be stopped when device is unplugged
    /// they will be started again when device is plugged in
    pub system_services_to_stop: Vec<String>,

    #[serde(default = "Vec::new")]
    /// User services which will be stopped when device is unplugged
    /// they will be started again when device is plugged in
    pub user_services_to_stop: Vec<String>,

    #[serde(default = "Vec::new")]
    /// Apps which will be stopped when device is unplugged
    /// they won't be started again when device is plugged in
    pub apps_to_stop: Vec<String>,

    /// Username to use when running commands
    /// for user units
    pub username: Option<String>,

    /// Commands to run when device is unplugged
    pub commands_unplugged: Option<Vec<String>>,

    /// Commands to run when device is plugged in
    pub commands_plugged: Option<Vec<String>>,
}

impl Config {
    pub fn new(args: &Arguments) -> Self {
        let config_path = args
            .config_file
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| CONFIG_FILE.to_path_buf());

        tracing::info!("Using config file: {:?}", config_path);

        let contents = fs::read_to_string(&config_path).unwrap_or_else(|_| {
            tracing::warn!("Could not read config file: {:?}", config_path);
            String::new()
        });

        toml::from_str(&contents).expect("Could not parse config file")
    }
}
