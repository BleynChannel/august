use august_plugin_system::{
    context::LoadPluginContext,
    utils::{ManagerResult, Ptr},
    Info, Loader, Manager, Plugin, RegisterPluginContext,
};

use crate::utils::config::{load_config, Config};

pub struct VoidPluginManager {
    configs: Vec<Config>,
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

    fn register_plugin(&mut self, context: RegisterPluginContext) -> ManagerResult<Info> {
        let (config, info) = load_config(context.path)?;
        self.configs.push(config);

        println!("VoidPluginManager::register_plugin - {}", context.bundle);
        Ok(info)
    }

    fn unregister_plugin(&mut self, plugin: &Plugin<'a, T>) -> ManagerResult<()> {
        println!(
            "VoidPluginManager::unregister_plugin - {}",
            plugin.info().bundle
        );
        Ok(())
    }

    fn load_plugin(&mut self, context: LoadPluginContext<'a, '_, T>) -> ManagerResult<()> {
        println!(
            "VoidPluginManager::load_plugin - {}",
            context.plugin().info().bundle
        );
        Ok(())
    }

    fn unload_plugin(&mut self, plugin: &Plugin<'a, T>) -> ManagerResult<()> {
        println!(
            "VoidPluginManager::unload_plugin - {}",
            plugin.info().bundle
        );
        Ok(())
    }
}

impl VoidPluginManager {
    pub fn new() -> Self {
        Self { configs: vec![] }
    }
}
