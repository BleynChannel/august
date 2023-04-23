use std::collections::HashMap;

use crate::{plugin_manager::PluginManager, LoadPluginError, Plugin, RegisterPluginError};

pub struct PluginLoader {
    plugin_managers: HashMap<String, Box<dyn PluginManager>>,
    plugins: Vec<Plugin>,
}

impl PluginLoader {
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
		self.plugin_managers.iter_mut().last().unwrap().1.register_manager();
    }

    pub fn register_plugin_managers(&mut self, managers: HashMap<String, Box<dyn PluginManager>>) {
        self.plugin_managers = managers;

		self.plugin_managers.iter_mut().for_each(|(_, manager)| {
			manager.as_mut().register_manager();
		});
    }

    pub fn unregister_plugin_manager(&mut self, filter: String) {
        self.plugin_managers.remove(&filter).unwrap().as_mut().unregister_manager();
    }

    pub fn get_plugin_manager(&self, format: String) -> Option<&Box<dyn PluginManager>> {
        self.plugin_managers.get(&format)
    }

    pub fn register_plugin(&mut self, path: &str) -> Result<&Plugin, RegisterPluginError> {
		Err(RegisterPluginError::NotFound)
    }

	pub fn load_plugin(&mut self, plugin: &Plugin) -> Result<(), LoadPluginError> {
		Err(LoadPluginError::UnknownManagerFormat(""))
	}

	pub fn load_plugin_now(&mut self, path: &str) -> Result<(), (Option<RegisterPluginError>, Option<LoadPluginError>)> {
		Err((None, None))
	}
}
