use std::{any::Any, fmt::Debug, path::PathBuf};

use crate::{
    function::Function,
    utils::{FunctionResult, Ptr},
    variable::Variable,
    Manager, PluginInfo,
};

pub struct Plugin<'a> {
    pub(crate) manager: Ptr<'a, Box<dyn Manager<'a>>>,
    pub(crate) path: PathBuf,
    pub(crate) info: PluginInfo,
    pub(crate) is_load: bool,
    pub(crate) requests: Vec<(Vec<Box<dyn Any>>, Function)>,
}

impl PartialEq for Plugin<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.info.id == other.info.id
    }
}

impl Debug for Plugin<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plugin").field("id", &self.info.id).finish()
    }
}

impl<'a> Plugin<'a> {
    pub(crate) const fn new(
        manager: Ptr<'a, Box<dyn Manager<'a>>>,
        path: PathBuf,
        info: PluginInfo,
    ) -> Self {
        Self {
            manager,
            path,
            info,
            is_load: false,
            requests: vec![],
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn info(&self) -> &PluginInfo {
        &self.info
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
