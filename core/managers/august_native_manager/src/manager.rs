use std::{env::consts::OS, path::PathBuf};

use crate::{config::NativeConfig, Plugin};
use august_plugin_system::{
    context::LoadPluginContext, utils::FunctionResult, Plugin as AugustPlugin, PluginInfo,
    PluginManager, WrapperLoader,
};
use libloading::Library;

pub struct NativePluginManager {
    plugins: Vec<Plugin>,
}

impl NativePluginManager {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            plugins: Vec::new(),
        })
    }

    fn remove_plugin(&mut self, info: &PluginInfo) {
        if let Some(index) = self.plugins.iter().enumerate().find_map(|(index, plugin)| {
            if plugin.info == *info {
                return Some(index);
            }
            None
        }) {
            self.plugins.remove(index);
        }
    }
}

impl PluginManager for NativePluginManager {
    fn format(&self) -> &str {
        "npl"
    }

    fn register_manager(&mut self, _: WrapperLoader) -> FunctionResult<()> {
        Ok(())
    }
    fn unregister_manager(&mut self) -> FunctionResult<()> {
        Ok(())
    }

    fn register_plugin(&mut self, path: &PathBuf) -> FunctionResult<PluginInfo> {
        let config = NativeConfig::load(path)?;
        let info = PluginInfo {
            id: config.id.clone(),
            depends: config.depends.clone().map_or(Vec::new(), |v| v.clone()),
            optional_depends: config
                .optional_depends
                .clone()
                .map_or(Vec::new(), |v| v.clone()),
        };

        self.plugins.push(Plugin::new(info.clone(), config));
        Ok(info)
    }
    fn unregister_plugin(&mut self, plugin: &AugustPlugin) -> FunctionResult<()> {
        self.remove_plugin(&plugin.info());
        Ok(())
    }

    fn register_plugin_error(&mut self, info: &PluginInfo) {
        self.remove_plugin(info);
    }

    fn load_plugin(&mut self, context: LoadPluginContext) -> FunctionResult<()> {
        let plugin = context.plugin();

        // Загрузка библиотеки
        #[cfg(target_os = "windows")]
        let script = "main.dll";
        #[cfg(target_os = "linux")]
        let script = "libmain.so";
        //TODO: Сделать для MacOS

        let library;
        unsafe {
            library = Library::new(
                plugin
                    .path()
                    .join(OS.to_string() + "/" + script)
                    .as_os_str(),
            )?;
        }

        let info = plugin.info();
        self.plugins
            .iter_mut()
            .find(|p| p.info == info)
            .unwrap()
            .library = Some(library);

        Ok(())
    }

    fn unload_plugin(&mut self, plugin: &AugustPlugin) -> FunctionResult<()> {
        let info = plugin.info();
        self.plugins
            .iter_mut()
            .find(|p| p.info == info)
            .unwrap()
            .library
            .take();

        Ok(())
    }
}
