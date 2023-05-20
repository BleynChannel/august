use std::{fs, path::PathBuf};

use august_plugin_system::utils::FunctionResult;
use serde::{Deserialize, Serialize};

use crate::error::RegisterPluginError;

#[derive(Debug, Deserialize, Serialize)]
pub struct NativeConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub license: Option<String>,
    pub depends: Option<Vec<String>>,
    pub optional_depends: Option<Vec<String>>,
}

impl NativeConfig {
    pub fn load(plugin_path: &PathBuf) -> FunctionResult<NativeConfig> {
        let config_path = plugin_path.join("config.toml");
		println!("{:?}", config_path.canonicalize().unwrap());
        if !config_path.exists() {
            return Err(Box::new(RegisterPluginError::DoesNotContainConfig));
        }

        let config_content = fs::read_to_string(config_path)?;
        Ok(toml::from_str::<NativeConfig>(&config_content)?)
    }
}
