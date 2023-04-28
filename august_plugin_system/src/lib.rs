mod config;
mod error;

pub use error::*;

pub use config::Config;
pub use config::Plugin as ConfigPlugin;

use std::cell::{Ref, RefCell, RefMut};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub struct Plugin {
    manager: Option<Rc<RefCell<Box<dyn PluginManager>>>>,
    path: PathBuf,
    config: Config,
    is_load: bool,
}

impl Plugin {
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn is_load(&self) -> bool {
        self.is_load
    }
}

pub trait PluginManager {
    fn format(&self) -> &str;

    fn register_manager(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    fn unregister_manager(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn register_plugin(&mut self, plugin: &Plugin) -> anyhow::Result<()> {
        Ok(())
    }
    fn unregister_plugin(&mut self, plugin: &Plugin) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct PluginLoader {
    plugin_managers: Vec<Rc<RefCell<Box<dyn PluginManager>>>>,
    plugins: Vec<Plugin>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            plugin_managers: Vec::new(),
            plugins: Vec::new(),
        }
    }

    pub fn init(managers: Vec<Box<dyn PluginManager>>) -> Result<Self, RegisterManagerError> {
        let mut loader = Self::new();

        for manager in managers {
            loader.register_plugin_manager(manager)?;
        }

        Ok(loader)
    }

    pub fn register_plugin_manager(
        &mut self,
        manager: Box<dyn PluginManager>,
    ) -> Result<(), RegisterManagerError> {
        if let Some(_) = self
            .plugin_managers
            .iter()
            .find(|m| manager.format() == m.borrow().format())
        {
            return Err(RegisterManagerError::AlreadyOccupiedFormat(
                manager.format().to_string(),
            ));
        }

        self.plugin_managers.push(Rc::new(RefCell::new(manager)));
        let manager = self.plugin_managers.last().unwrap();
        manager.borrow_mut().register_manager()?;
        Ok(())
    }

    pub fn register_plugin_managers(
        &mut self,
        managers: Vec<Box<dyn PluginManager>>,
    ) -> Result<(), RegisterManagerError> {
        for manager in managers {
            self.register_plugin_manager(manager)?;
        }

        Ok(())
    }

    pub fn unregister_plugin_manager(
        &mut self,
        format: String,
    ) -> Result<(), UnregisterManagerError> {
        if let Some(manager) = self
            .plugin_managers
            .iter()
            .find(|m| m.borrow().format() == format)
        {
            manager.borrow_mut().unregister_manager()?;
            Ok(())
        } else {
            return Err(UnregisterManagerError::NotFound(format));
        }
    }

    pub fn get_plugin_manager(&self, format: String) -> Option<Ref<'_, Box<dyn PluginManager>>> {
        self.plugin_managers.iter().find_map(|m| {
            if m.borrow().format() == format {
                return Some(m.borrow());
            }
            None
        })
    }

    pub fn get_plugin_manager_mut(
        &mut self,
        format: String,
    ) -> Option<RefMut<'_, Box<dyn PluginManager>>> {
        self.plugin_managers.iter().find_map(|m| {
            if m.borrow().format() == format {
                return Some(m.borrow_mut());
            }
            None
        })
    }

    pub fn register_plugin(&mut self, path: &str) -> Result<String, RegisterPluginError> {
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
            if let Some(manager) = self
                .plugin_managers
                .iter()
                .find(|manager| manager.borrow().format() == plugin_format)
            {
                // Получаю конфигурацию плагина
                let config_path = path.join("config.toml");
                if !config_path.exists() {
                    return Err(RegisterPluginError::DoesNotContainConfig);
                }

                let config_content = fs::read_to_string(config_path)?;
                let config = toml::from_str::<Config>(&config_content)?;

                if self
                    .plugins
                    .iter()
                    .find(|p| p.config.plugin.name == config.plugin.name)
                    .is_some()
                {
                    return Err(RegisterPluginError::AlreadyExistsName(
                        config.plugin.name.clone(),
                    ));
                }

                // Регистрирую плагин
                self.plugins.push(Plugin {
                    manager: None,
                    path: path.clone(),
                    config,
                    is_load: false,
                });
                let plugin = self.plugins.last_mut().unwrap();

                manager.borrow_mut().register_plugin(plugin)?;
                plugin.manager = Some(manager.clone());

                return Ok(plugin.config.plugin.name.clone());
            } else {
                return Err(RegisterPluginError::UnknownManagerFormat(
                    plugin_format.to_string(),
                ));
            }
        } else {
            return Err(RegisterPluginError::UnknownManagerFormat("".to_string()));
        }
    }

    pub fn unregister_plugin(&mut self, plugin_name: &str) -> Result<(), UnregisterPluginError> {
        if let Some(plugin) = self
            .plugins
            .iter_mut()
            .find(|p| p.config.plugin.name == plugin_name)
        {
            if plugin.is_load {
                return Err(UnregisterPluginError::IsLoaded);
            }

            let manager = plugin.manager.as_ref().unwrap();
            if let Some(manager) = self
                .plugin_managers
                .iter()
                .find(|m| m.borrow().format() == manager.borrow().format())
            {
                manager.borrow_mut().unregister_plugin(plugin)?;
            } else {
                return Err(UnregisterPluginError::HasUnregisteredManager);
            }

            self.plugins.retain(|p| p.config.plugin.name != plugin_name);
        } else {
            return Err(UnregisterPluginError::NotFound);
        }

        return Ok(());
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

    pub fn get_plugin(&self, name: &str) -> Option<&Plugin> {
        self.plugins
            .iter()
            .find(|plugin| plugin.get_config().plugin.name == name)
    }

    pub fn get_plugin_mut(&mut self, name: &str) -> Option<&mut Plugin> {
        self.plugins
            .iter_mut()
            .find(|plugin| plugin.get_config().plugin.name == name)
    }
}
