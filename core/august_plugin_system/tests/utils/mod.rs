mod native_config;
mod managers;
pub use managers::*;

pub use native_config::*;

use std::path::PathBuf;

use august_plugin_system::{LoaderBuilder, PluginLoader, PluginManager};

pub fn get_plugin_path(name: &str, format: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join(format!("../../plugins/{name}/plugin.{format}"))
}

pub fn loader_init(manager: Box<dyn PluginManager>) -> PluginLoader {
    match LoaderBuilder::new().register_manager(manager).build() {
        Ok(loader) => loader,
        Err(e) => {
            panic!("{:?}: {}", e, e.to_string())
        }
    }
}
