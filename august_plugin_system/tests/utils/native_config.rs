use std::{fs, path::PathBuf};

use august_plugin_system::PluginInfo;
use serde::{Deserialize, Serialize};

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

#[derive(thiserror::Error, Debug)]
pub enum RegisterPluginError {
    #[error("Does not contain config")]
    DoesNotContainConfig,
}

pub fn load_config(plugin_path: &PathBuf) -> anyhow::Result<(NativeConfig, PluginInfo)> {
	// Получаем конфигурацию плагина
	let config_path = plugin_path.join("config.toml");
	if !config_path.exists() {
		return Err(anyhow::Error::from(
			RegisterPluginError::DoesNotContainConfig,
		));
	}

	let config_content = fs::read_to_string(config_path)?;
	let config = toml::from_str::<NativeConfig>(&config_content)?;

	//Заполняем информацию про плагин
	let info = PluginInfo {
		id: config.id.clone(),
		depends: config.depends.clone().map_or(Vec::new(), |v| v.clone()),
		optional_depends: config
			.optional_depends
			.clone()
			.map_or(Vec::new(), |v| v.clone()),
	};

	Ok((config, info))
}