use std::{cmp::Ordering, fmt::Debug};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;

use crate::{
    function::Function,
    utils::{bundle::Bundle, PluginCallRequest, Ptr},
    variable::Variable,
    Depend, Manager, PluginInfo,
};

//TODO: Добавить реестр функций для плагинов
pub struct Plugin<'a, T: Send + Sync> {
    pub(crate) manager: Ptr<'a, Box<dyn Manager<'a, T>>>,
    pub(crate) info: PluginInfo,
    pub(crate) is_load: bool,
    pub(crate) requests: Vec<Box<dyn Function<Output = T>>>,
}

impl<'a, T: Send + Sync> Plugin<'a, T> {
    pub(crate) const fn new(manager: Ptr<'a, Box<dyn Manager<'a, T>>>, info: PluginInfo) -> Self {
        Self {
            manager,
            info,
            is_load: false,
            requests: vec![],
        }
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

impl<T: Send + Sync> PartialEq for Plugin<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.info.bundle.id == other.info.bundle.id
            && self.info.bundle.version == other.info.bundle.version
    }
}

impl<T: Send + Sync, ID: AsRef<str>> PartialEq<(ID, &Version)> for Plugin<'_, T> {
    fn eq(&self, (id, version): &(ID, &Version)) -> bool {
        self.info.bundle.id == *id.as_ref() && self.info.bundle.version == **version
    }
}

impl<T: Send + Sync> PartialEq<Bundle> for Plugin<'_, T> {
    fn eq(&self, Bundle { id, version, .. }: &Bundle) -> bool {
        self.info.bundle.id == *id && self.info.bundle.version == *version
    }
}

impl<T: Send + Sync> PartialEq<Depend> for Plugin<'_, T> {
    fn eq(&self, Depend { id: name, version }: &Depend) -> bool {
        self.info.bundle.id == *name && self.info.bundle.version == *version
    }
}

impl<T: Send + Sync> Eq for Plugin<'_, T> {}

impl<T: Send + Sync> PartialOrd for Plugin<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.info.bundle.id == other.info.bundle.id {
            true => self
                .info
                .bundle
                .version
                .partial_cmp(&other.info.bundle.version),
            false => None,
        }
    }
}

impl<T: Send + Sync, ID: AsRef<str>> PartialOrd<(ID, &Version)> for Plugin<'_, T> {
    fn partial_cmp(&self, (id, version): &(ID, &Version)) -> Option<Ordering> {
        match self.info.bundle.id == *id.as_ref() {
            true => self.info.bundle.version.partial_cmp(*version),
            false => None,
        }
    }
}

impl<T: Send + Sync> PartialOrd<Bundle> for Plugin<'_, T> {
    fn partial_cmp(&self, Bundle { id, version, .. }: &Bundle) -> Option<Ordering> {
        match self.info.bundle.id == *id {
            true => self.info.bundle.version.partial_cmp(version),
            false => None,
        }
    }
}

impl<T: Send + Sync> PartialOrd<Depend> for Plugin<'_, T> {
    fn partial_cmp(&self, Depend { id: name, version, .. }: &Depend) -> Option<Ordering> {
        match self.info.bundle.id == *name {
            true => self.info.bundle.version.partial_cmp(version),
            false => None,
        }
    }
}

impl<T: Send + Sync> Debug for Plugin<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plugin")
            .field("id", &self.info.bundle.id)
            .field("version", &self.info.bundle.version)
            .field("format", &self.info.bundle.format)
            .field("path", &self.info.path)
            .field("is_load", &self.is_load)
            .field("depends", &self.info.info.depends)
            .field("optional_depends", &self.info.info.optional_depends)
            .finish()
    }
}
