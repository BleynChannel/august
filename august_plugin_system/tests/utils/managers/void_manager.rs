use std::path::PathBuf;

use august_plugin_system::{Link, Plugin, PluginInfo, PluginManager};

use crate::utils::native_config::{load_config, NativeConfig};

pub struct VoidPluginManager {
    configs: Vec<NativeConfig>,
}

impl PluginManager for VoidPluginManager {
    fn format(&self) -> &str {
        "vpl"
    }

    fn register_manager(&mut self) -> anyhow::Result<()> {
        println!("VoidPluginManager::register_manager");
        Ok(())
    }

    fn unregister_manager(&mut self) -> anyhow::Result<()> {
        println!("VoidPluginManager::unregister_manager");
        Ok(())
    }

    fn register_plugin(&mut self, path: &PathBuf) -> anyhow::Result<PluginInfo> {
		let (config, info) = load_config(path)?;
        self.configs.push(config);

        println!("VoidPluginManager::register_plugin - {}", info.id);
        Ok(info)
    }

    fn unregister_plugin(&mut self, plugin: &Link<Plugin>) -> anyhow::Result<()> {
        println!(
            "VoidPluginManager::unregister_plugin - {:?}",
            plugin.borrow().get_path()
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

        println!("VoidPluginManager::register_plugin_error");
    }

    fn load_plugin(&mut self, plugin: &Link<Plugin>) -> anyhow::Result<()> {
        println!(
            "VoidPluginManager::load_plugin - {:?}",
            plugin.borrow().get_info().id
        );
        Ok(())
    }

    fn unload_plugin(&mut self, plugin: &Link<Plugin>) -> anyhow::Result<()> {
        println!(
            "VoidPluginManager::unload_plugin - {:?}",
            plugin.borrow().get_info().id
        );
        Ok(())
    }
}

impl VoidPluginManager {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            configs: Vec::new(),
        })
    }
}
