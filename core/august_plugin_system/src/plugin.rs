use std::{any::Any, fmt::Debug, path::PathBuf};

use crate::{
    function::Function, utils::FunctionResult, variable::Variable, Link, PluginInfo, PluginManager,
};

pub struct Plugin {
    pub(crate) manager: Link<Box<dyn PluginManager>>,
    pub(crate) path: PathBuf,
    pub(crate) info: PluginInfo,
    pub(crate) is_load: bool,
    pub(crate) requests: Vec<(Vec<Box<dyn Any>>, Function)>,
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
    pub(crate) fn new(
        manager: Link<Box<dyn PluginManager>>,
        path: PathBuf,
        info: PluginInfo,
    ) -> Self {
        Self {
            manager,
            path,
            info,
            is_load: false,
            requests: Vec::new(),
        }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn info(&self) -> PluginInfo {
        self.info.clone()
    }

    pub fn is_load(&self) -> bool {
        self.is_load
    }

    pub fn call_request(&self, name: &str, args: &[Variable]) -> FunctionResult<Option<Variable>> {
        self.requests
            .iter()
            .find_map(|(external, request)| match request.name == name {
                true => Some(request.call(external.as_slice(), args)),
                false => None,
            })
            .unwrap()
    }
}
