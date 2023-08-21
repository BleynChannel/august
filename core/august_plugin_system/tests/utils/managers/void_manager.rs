use std::path::PathBuf;

use august_plugin_system::{
    context::LoadPluginContext,
    utils::{ManagerResult, Ptr},
    Loader, Manager, Plugin, PluginInfo,
};

use crate::utils::native_config::{load_config, NativeConfig};

pub struct VoidPluginManager {
    configs: Vec<NativeConfig>,
}

impl<'a, T: Send + Sync> Manager<'a, T> for VoidPluginManager {
    fn format(&self) -> &str {
        "vpl"
    }

    fn register_manager(&mut self, _: Ptr<'a, Loader<'a, T>>) -> ManagerResult<()> {
        println!("VoidPluginManager::register_manager");
        Ok(())
    }

    fn unregister_manager(&mut self) -> ManagerResult<()> {
        println!("VoidPluginManager::unregister_manager");
        Ok(())
    }

    fn register_plugin(&mut self, path: &PathBuf) -> ManagerResult<PluginInfo> {
        let (config, info) = load_config(path)?;
        self.configs.push(config);

        println!("VoidPluginManager::register_plugin - {}", info.id);
        Ok(info)
    }

    fn unregister_plugin(&mut self, plugin: Ptr<'a, Plugin<'a, T>>) -> ManagerResult<()> {
        println!(
            "VoidPluginManager::unregister_plugin - {:?}",
            plugin.as_ref().path()
        );
        Ok(())
    }

    fn register_plugin_error(&mut self, info: PluginInfo) {
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

    fn load_plugin(&mut self, context: LoadPluginContext<'a, T>) -> ManagerResult<()> {
        println!(
            "VoidPluginManager::load_plugin - {:?}",
            context.plugin().info().id
        );
        Ok(())
    }

    fn unload_plugin(&mut self, plugin: Ptr<'a, Plugin<'a, T>>) -> ManagerResult<()> {
        println!(
            "VoidPluginManager::unload_plugin - {:?}",
            plugin.as_ref().info().id
        );
        Ok(())
    }
}

impl VoidPluginManager {
    pub fn new() -> Self {
        Self { configs: vec![] }
    }
}
