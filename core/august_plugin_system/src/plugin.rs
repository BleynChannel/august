use std::{fmt::Debug, path::PathBuf};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    function::Function,
    utils::{PluginCallRequest, Ptr},
    variable::Variable,
    Manager, PluginInfo,
};

pub struct Plugin<'a, T: Send + Sync> {
    pub(crate) manager: Ptr<'a, Box<dyn Manager<'a, T>>>,
    pub(crate) path: PathBuf,
    pub(crate) info: PluginInfo,
    pub(crate) is_load: bool,
    pub(crate) requests: Vec<Box<dyn Function<Output = T>>>,
}

impl<F: Function> PartialEq for Plugin<'_, F> {
    fn eq(&self, other: &Self) -> bool {
        self.info.id == other.info.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.info.id != other.info.id
    }
}

impl<T: Send + Sync> Debug for Plugin<'_, T> {
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

impl<'a, T: Send + Sync> Plugin<'a, T> {
    pub(crate) const fn new(
        manager: Ptr<'a, Box<dyn Manager<'a, T>>>,
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

    pub const fn get_requests(&self) -> &Vec<Box<dyn Function<Output = T>>> {
        &self.requests
    }

    pub fn call_request(&self, name: &str, args: &[Variable]) -> Result<T, PluginCallRequest> {
        self.requests
            .iter()
            .find_map(|request| match request.name() == name {
                true => Some(request.call(args)),
                false => None,
            })
            .ok_or(PluginCallRequest::NotFound)
    }

    pub fn call_requests(&self, args: &[Variable]) -> Vec<T> {
        self.requests
            .iter()
            .map(|request| request.call(&args))
            .collect()
    }

    pub fn par_call_requests(&self, args: &[Variable]) -> Vec<T> {
        self.requests
            .par_iter()
            .map(|request| request.call(&args))
            .collect()
    }
}
