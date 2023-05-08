pub mod native_config;
pub mod managers;

use std::path::PathBuf;

use august_plugin_system::{PluginLoader, PluginManager};

pub fn get_void_plugin_path(name: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join(format!("../plugins/{name}/plugin.vpl"))
}

pub fn get_native_plugin_path(name: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join(format!("../plugins/{name}/target/debug/plugin.npl"))
}

pub fn loader_init(manager: Box<dyn PluginManager>) -> PluginLoader
{
    let mut plugin_managers: Vec<Box<dyn PluginManager>> = Vec::new();
    plugin_managers.push(manager);

    match PluginLoader::init(plugin_managers) {
        Ok(loader) => loader,
        Err(e) => {
            panic!("{:?}: {}", e, e.to_string())
        }
    }
}
