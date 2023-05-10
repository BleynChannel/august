use std::{path::PathBuf, fmt::Debug};

use crate::{Link, PluginInfo, PluginManager};

pub struct Plugin {
    pub(crate) manager: Link<Box<dyn PluginManager>>,
    pub(crate) path: PathBuf,
    pub(crate) info: PluginInfo,
    pub(crate) is_load: bool,
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
}
