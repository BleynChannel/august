mod native_config;
mod void_manager;

pub use native_config::*;
pub use void_manager::*;

use std::path::PathBuf;

use august_plugin_system::{PluginLoader, PluginManager};

pub fn get_plugin_path(name: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join(format!("../../plugins/{name}/plugin.vpl"))
}

pub fn loader_init(manager: Box<dyn PluginManager>) -> PluginLoader {
    let mut plugin_managers: Vec<Box<dyn PluginManager>> = Vec::new();
    plugin_managers.push(manager);

    match PluginLoader::init(plugin_managers) {
        Ok(loader) => loader,
        Err(e) => {
            panic!("{:?}: {}", e, e.to_string())
        }
    }
}
