use std::{fs, path::PathBuf};

use august_plugin_system::{Plugin, PluginInfo, PluginManager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NativeConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub license: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum RegisterPluginError {
    #[error("Does not contain config")]
    DoesNotContainConfig,
}

pub struct TestManagerPlugin {
    configs: Vec<NativeConfig>,
}

impl PluginManager for TestManagerPlugin {
    fn format(&self) -> &str {
        "testpl"
    }

    fn register_manager(&mut self) -> anyhow::Result<()> {
        println!("TestManagerPlugin::register_manager");
        Ok(())
    }

    fn unregister_manager(&mut self) -> anyhow::Result<()> {
        println!("TestManagerPlugin::unregister_manager");
        Ok(())
    }

    fn register_plugin(&mut self, path: &PathBuf) -> anyhow::Result<PluginInfo> {
        // Получаем конфигурацию плагина
        let config_path = path.join("config.toml");
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
        };
        self.configs.push(config);

        println!("TestManagerPlugin::register_plugin - {}", info.id);
        Ok(info)
    }

    fn unregister_plugin(&mut self, plugin: &Plugin) -> anyhow::Result<()> {
        println!(
            "TestManagerPlugin::unregister_plugin - {:?}",
            plugin.get_path()
        );
        Ok(())
    }

    fn register_plugin_error(&mut self, info: &PluginInfo) {
        if let Some(index) = self.configs.iter().enumerate().find_map(|(index, config)| {
            if config.id == info.id {
                return Some(index);
            }
            None
        }) {
            self.configs.remove(index);
        }

        println!("TestManagerPlugin::register_plugin_error");
    }
}

impl TestManagerPlugin {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            configs: Vec::new(),
        })
    }
}
