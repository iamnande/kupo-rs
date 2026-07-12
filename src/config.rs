use serde::Deserialize;
use std::path::PathBuf;

use crate::error::Result;
use crate::stash::Stash;

#[derive(Debug, Deserialize)]
pub struct KupoConfig {
    pub stash: Stash,
}

const CONFIG_DIR: &str = ".config/kupo";
const CONFIG_FILE: &str = "config.toml";

pub fn load_config() -> Result<KupoConfig> {
    let home = if let Ok(user) = std::env::var("SUDO_USER") {
        format!("/home/{user}")
    } else {
        std::env::var("HOME")?
    };
    let config_path = PathBuf::from(home).join(CONFIG_DIR).join(CONFIG_FILE);

    let raw = std::fs::read_to_string(config_path)?;
    let config = toml::from_str(&raw)?;
    Ok(config)
}
