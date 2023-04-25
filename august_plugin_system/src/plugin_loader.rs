use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

use crate::{plugin_manager::PluginManager, Config, LoadPluginError, Plugin, RegisterPluginError};

pub struct PluginLoader<'a> {
    plugin_managers: HashMap<String, Box<dyn PluginManager>>,
    plugins: Vec<Plugin<'a>>,
}

impl<'a> PluginLoader<'a> {
    pub fn new() -> Self {
        Self {
            plugin_managers: HashMap::new(),
            plugins: Vec::new(),
        }
    }

    pub fn init(managers: HashMap<String, Box<dyn PluginManager>>) -> Self {
        let mut loader = Self {
            plugins: Vec::new(),
            plugin_managers: managers,
        };

        loader.plugin_managers.iter_mut().for_each(|(_, manager)| {
            manager.as_mut().register_manager();
        });

        loader
    }

    pub fn register_plugin_manager(&mut self, format: String, manager: Box<dyn PluginManager>) {
        self.plugin_managers.insert(format, manager);
        self.plugin_managers
            .iter_mut()
            .last()
            .unwrap()
            .1
            .register_manager();
    }

    pub fn register_plugin_managers(&mut self, managers: HashMap<String, Box<dyn PluginManager>>) {
        self.plugin_managers = managers;

        self.plugin_managers.iter_mut().for_each(|(_, manager)| {
            manager.as_mut().register_manager();
        });
    }

    pub fn unregister_plugin_manager(&mut self, filter: String) {
        self.plugin_managers
            .remove(&filter)
            .unwrap()
            .as_mut()
            .unregister_manager();
    }

    pub fn get_plugin_manager(&self, format: String) -> Option<&Box<dyn PluginManager>> {
        self.plugin_managers.get(&format)
    }

    pub fn register_plugin(&'a mut self, path: &str) -> Result<&Plugin, RegisterPluginError> {
        let path = Path::new(path).to_path_buf();

        if !path.exists() {
            return Err(RegisterPluginError::NotFound);
        }
        if !path.is_dir() {
            return Err(RegisterPluginError::UnpackError(
                "Not a directory".to_string(),
            ));
        }

        // Получаю формат плагина и ищу подходящий менеджер
        if let Some(plugin_format) = path.extension() {
            let plugin_format = plugin_format.to_str().unwrap();

            for (format, manager) in self.plugin_managers.iter_mut() {
                if plugin_format == format {
                    // Получаю конфигурацию плагина
                    let config_path = path.join("config.toml");
                    if !config_path.exists() {
                        return Err(RegisterPluginError::DoesNotContainConfig);
                    }

                    let config_content = fs::read_to_string(config_path)?;
                    let config = toml::from_str::<Config>(&config_content)?;

                    // Регистрирую плагин
                    let manager_mut = manager.as_mut();

                    self.plugins.push(Plugin::_new(path, config));
                    let plugin = self.plugins.last_mut().unwrap();

                    manager_mut.register_plugin(plugin)?;
                    plugin._set_manager(manager_mut);

                    return Ok(plugin);
                }
            }

            return Err(RegisterPluginError::UnknownManagerFormat(
                plugin_format.to_string(),
            ));
        } else {
            return Err(RegisterPluginError::UnknownManagerFormat("".to_string()));
        }
    }

    pub fn load_plugin(&mut self, plugin: &Plugin) -> Result<(), LoadPluginError> {
        Err(LoadPluginError::UnknownManagerFormat("".to_string()))
    }

    pub fn load_plugin_now(
        &mut self,
        path: &str,
    ) -> Result<(), (Option<RegisterPluginError>, Option<LoadPluginError>)> {
        Err((None, None))
    }
}
