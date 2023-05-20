use std::{env::consts::OS, path::PathBuf};

use crate::{config::NativeConfig, Plugin};
use august_plugin_system::{
    utils::FunctionResult, Plugin as AugustPlugin, PluginInfo, PluginManager, WrapperLoader,
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

        // Загрузка библиотеки
        #[cfg(target_os = "windows")]
        let script = "main";
        #[cfg(target_os = "linux")]
        let script = "libmain";
        //TODO: Сделать для MacOS

        let test_path = path.join(OS.to_string() + "/");
        for entry in std::fs::read_dir(test_path).expect("Read dir error") {
			let entry = entry.unwrap();
			println!("{:?}", entry.file_name())
		}

        let library;
        unsafe {
            library = Library::new(path.join(OS.to_string() + "/" + script).as_os_str())?;
        }

        self.plugins
            .push(Plugin::new(info.clone(), config, library));
        Ok(info)
    }
    fn unregister_plugin(&mut self, plugin: &AugustPlugin) -> FunctionResult<()> {
        self.remove_plugin(&plugin.get_info());
        Ok(())
    }

    fn register_plugin_error(&mut self, info: &PluginInfo) {
        self.remove_plugin(info);
    }

    fn load_plugin(&mut self, plugin: &AugustPlugin) -> FunctionResult<()> {
        Ok(())
    }

    fn unload_plugin(&mut self, plugin: &AugustPlugin) -> FunctionResult<()> {
        Ok(())
    }
}
