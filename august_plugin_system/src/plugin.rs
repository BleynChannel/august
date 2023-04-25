use std::path::PathBuf;

use crate::{config::Config, PluginManager};

pub struct Plugin<'a> {
    manager: Option<&'a dyn PluginManager>,
    path: PathBuf,
    config: Config,
    is_load: bool,
}

impl<'a> Plugin<'a> {
    pub fn _new(path: PathBuf, config: Config) -> Self {
        Self {
            manager: None,
            path,
            config,
            is_load: false,
        }
    }

    pub fn _set_manager(&mut self, manager: &'a dyn PluginManager) {
        self.manager = Some(manager);
    }

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
