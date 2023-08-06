use std::{fmt::Debug, path::PathBuf};

use crate::{
    function::Function,
    utils::{PluginCallRequest, Ptr},
    variable::Variable,
    Manager, PluginInfo,
};

pub struct Plugin<'a, F: Function> {
    pub(crate) manager: Ptr<'a, Box<dyn Manager<'a, F>>>,
    pub(crate) path: PathBuf,
    pub(crate) info: PluginInfo,
    pub(crate) is_load: bool,
    pub(crate) requests: Vec<F>,
}

impl<F: Function> PartialEq for Plugin<'_, F> {
    fn eq(&self, other: &Self) -> bool {
        self.info.id == other.info.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.info.id != other.info.id
    }
}

impl<F: Function> Debug for Plugin<'_, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plugin")
            .field("id", &self.info.id)
            .field("path", &self.path)
            .field("is_load", &self.is_load)
            .field("depends", &self.info.depends)
            .field("optional_depends", &self.info.optional_depends)
            .finish()
    }
}

impl<'a, F: Function> Plugin<'a, F> {
    pub(crate) const fn new(
        manager: Ptr<'a, Box<dyn Manager<'a, F>>>,
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

    pub const fn path(&self) -> &PathBuf {
        &self.path
    }

    pub const fn info(&self) -> &PluginInfo {
        &self.info
    }

    pub const fn is_load(&self) -> bool {
        self.is_load
    }

    pub fn call_request(
        &self,
        name: &str,
        args: &[Variable],
    ) -> Result<F::CallResult, PluginCallRequest> {
        self.requests
            .iter()
            .find_map(|request| match request.name() == name {
                true => Some(request.call(args)),
                false => None,
            })
            .ok_or(PluginCallRequest::NotFound)
    }
}
