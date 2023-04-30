mod error;
pub use error::*;

use serde::{Deserialize, Serialize};

use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginInfo {
    pub id: String,
}

pub struct Plugin {
    manager: Rc<RefCell<Box<dyn PluginManager>>>,
    path: PathBuf,
    info: PluginInfo,
    is_load: bool,
}

impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.info.id == other.info.id
    }
}

impl Debug for Plugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plugin").field("id", &self.info.id).finish()
    }
}

impl Plugin {
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_info(&self) -> &PluginInfo {
        &self.info
    }

    pub fn is_load(&self) -> bool {
        self.is_load
    }

    pub fn load(&mut self) -> Result<(), LoadPluginError> {
        self.is_load = true;
        Ok(())
    }

    pub fn unload(&mut self) -> Result<(), UnloadPluginError> {
        self.is_load = false;
        Ok(())
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

    fn register_plugin(&mut self, path: &PathBuf) -> anyhow::Result<PluginInfo> {
        Ok(PluginInfo { id: path.file_name().unwrap().to_str().unwrap().to_string() })
    }
    fn unregister_plugin(&mut self, plugin: &Plugin) -> anyhow::Result<()> {
        Ok(())
    }

    fn register_plugin_error(&mut self, info: &PluginInfo) {}
}

impl PartialEq for dyn PluginManager {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
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
            loader.register_manager(manager)?;
        }

        Ok(loader)
    }

    pub fn register_manager(
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

    pub fn register_managers(
        &mut self,
        managers: Vec<Box<dyn PluginManager>>,
    ) -> Result<(), RegisterManagerError> {
        for manager in managers {
            self.register_manager(manager)?;
        }

        Ok(())
    }

    pub fn unregister_manager(&mut self, index: usize) -> Result<(), UnregisterManagerError> {
        if let Some(manager) = self.plugin_managers.get(index) {
            manager.borrow_mut().unregister_manager()?;
            self.plugin_managers.remove(index);
            Ok(())
        } else {
            return Err(UnregisterManagerError::NotFound);
        }
    }

    pub fn get_manager(&self, index: usize) -> Option<Ref<'_, Box<dyn PluginManager>>> {
        self.plugin_managers.get(index).map(|m| m.borrow())
    }

    pub fn get_manager_mut(&mut self, index: usize) -> Option<RefMut<'_, Box<dyn PluginManager>>> {
        self.plugin_managers.get(index).map(|m| m.borrow_mut())
    }

    pub fn get_managers(&self) -> Vec<Ref<'_, Box<dyn PluginManager>>> {
        self.plugin_managers.iter().map(|m| m.borrow()).collect()
    }

    pub fn register_plugin(&mut self, path: &str) -> Result<usize, RegisterPluginError> {
        let path = Path::new(path).to_path_buf();

        if !path.exists() {
            return Err(RegisterPluginError::NotFound);
        }
        if !path.is_dir() {
            return Err(RegisterPluginError::UnpackError(
                "Not a directory".to_string(),
            ));
        }

        // Получаем формат плагина и ищем подходящий менеджер
        if let Some(plugin_format) = path.extension() {
            let plugin_format = plugin_format.to_str().unwrap();
            if let Some(manager) = self
                .plugin_managers
                .iter()
                .find(|m| m.borrow().format() == plugin_format)
            {
                //Получаем нужную информацию про плагин
                let info = manager.borrow_mut().register_plugin(&path)?;

                if self.plugins.iter().find(|p| p.info.id == info.id).is_some() {
					manager.borrow_mut().register_plugin_error(&info);
                    return Err(RegisterPluginError::AlreadyExistsID(info.id.clone()));
                }

                // Регистрируем плагин
                self.plugins.push(Plugin {
                    manager: manager.clone(),
                    path: path.clone(),
                    info,
                    is_load: false,
                });

                return Ok(self.plugins.len() - 1);
            } else {
                return Err(RegisterPluginError::UnknownManagerFormat(
                    plugin_format.to_string(),
                ));
            }
        } else {
            return Err(RegisterPluginError::UnknownManagerFormat("".to_string()));
        }
    }

    pub fn unregister_plugin(&mut self, index: usize) -> Result<(), UnregisterPluginError> {
        if let Some(plugin) = self.plugins.get_mut(index) {
            if plugin.is_load {
                plugin.unload()?;
            }

            let manager = plugin.manager.as_ref();
            manager.borrow_mut().unregister_plugin(plugin)?;

            self.plugins.remove(index);
        } else {
            return Err(UnregisterPluginError::NotFound);
        }

        return Ok(());
    }

    // pub fn load_plugin(&mut self, plugin_name: &str) -> Result<(), LoadPluginError> {
    //     Err(LoadPluginError::NotFoundDependencies(&[]))
    // }

    // pub fn load_plugin_now(
    //     &mut self,
    //     path: &str,
    // ) -> Result<(), (Option<RegisterPluginError>, Option<LoadPluginError>)> {
    //     Err((None, None))
    // }

    pub fn get_plugin(&self, index: usize) -> Option<&Plugin> {
        self.plugins.get(index)
    }

    pub fn get_plugin_mut(&mut self, index: usize) -> Option<&mut Plugin> {
        self.plugins.get_mut(index)
    }
}
