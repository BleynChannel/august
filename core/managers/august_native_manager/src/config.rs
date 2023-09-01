use std::{collections::HashMap, fs, path::PathBuf};

use august_plugin_system::utils::ManagerResult;
use semver::VersionReq;
use serde::{Deserialize, Serialize};

use crate::error::RegisterPluginError;

#[derive(Debug, Deserialize, Serialize)]
pub struct NativeConfig {
    pub name: String,
    pub description: String,
    pub author: String,
    pub license: Option<String>,
    pub depends: Option<HashMap<String, VersionReq>>,
    pub optional_depends: Option<HashMap<String, VersionReq>>,
}

impl NativeConfig {
    pub fn load(plugin_path: &PathBuf) -> ManagerResult<NativeConfig> {
        let config_path = plugin_path.join("config.toml");
        if !config_path.exists() {
            return Err(Box::new(RegisterPluginError::DoesNotContainConfig));
        }

        let config_content = fs::read_to_string(config_path)?;
        Ok(toml::from_str::<NativeConfig>(&config_content)?)
    }
}
