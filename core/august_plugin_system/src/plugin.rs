use std::{cmp::Ordering, fmt::Debug};

use semver::Version;

use crate::{
    function::Function,
    utils::{bundle::Bundle, PluginCallRequest, Ptr},
    variable::Variable,
    Depend, Info, Manager, PluginInfo,
};

//TODO: Добавить реестр функций для плагинов
pub struct Plugin<'a, O: Send + Sync, I: Info> {
    pub(crate) manager: Ptr<'a, Box<dyn Manager<'a, O, I>>>,
    pub(crate) info: PluginInfo<I>,
    pub(crate) is_load: bool,
    pub(crate) requests: Vec<Box<dyn Function<Output = O>>>,
}

impl<'a, O: Send + Sync, I: Info> Plugin<'a, O, I> {
    pub(crate) const fn new(
        manager: Ptr<'a, Box<dyn Manager<'a, O, I>>>,
        info: PluginInfo<I>,
    ) -> Self {
        Self {
            manager,
            info,
            is_load: false,
            requests: vec![],
        }
    }

    pub const fn info(&self) -> &PluginInfo<I> {
        &self.info
    }

    pub const fn is_load(&self) -> bool {
        self.is_load
    }

    pub const fn get_requests(&self) -> &Vec<Box<dyn Function<Output = O>>> {
        &self.requests
    }

    pub fn call_request(&self, name: &str, args: &[Variable]) -> Result<O, PluginCallRequest> {
        self.requests
            .iter()
            .find_map(|request| match request.name() == name {
                true => Some(request.call(args)),
                false => None,
            })
            .ok_or(PluginCallRequest::NotFound)
    }
}

impl<O: Send + Sync, I: Info> PartialEq for Plugin<'_, O, I> {
    fn eq(&self, other: &Self) -> bool {
        self.info.bundle.id == other.info.bundle.id
            && self.info.bundle.version == other.info.bundle.version
    }
}

impl<O: Send + Sync, I: Info, ID: AsRef<str>> PartialEq<(ID, &Version)> for Plugin<'_, O, I> {
    fn eq(&self, (id, version): &(ID, &Version)) -> bool {
        self.info.bundle.id == *id.as_ref() && self.info.bundle.version == **version
    }
}

impl<O: Send + Sync, I: Info> PartialEq<Bundle> for Plugin<'_, O, I> {
    fn eq(&self, Bundle { id, version, .. }: &Bundle) -> bool {
        self.info.bundle.id == *id && self.info.bundle.version == *version
    }
}

impl<O: Send + Sync, I: Info> PartialEq<Depend> for Plugin<'_, O, I> {
    fn eq(&self, Depend { id: name, version }: &Depend) -> bool {
        self.info.bundle.id == *name && self.info.bundle.version == *version
    }
}

impl<O: Send + Sync, I: Info> Eq for Plugin<'_, O, I> {}

impl<O: Send + Sync, I: Info> PartialOrd for Plugin<'_, O, I> {
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

impl<O: Send + Sync, I: Info, ID: AsRef<str>> PartialOrd<(ID, &Version)> for Plugin<'_, O, I> {
    fn partial_cmp(&self, (id, version): &(ID, &Version)) -> Option<Ordering> {
        match self.info.bundle.id == *id.as_ref() {
            true => self.info.bundle.version.partial_cmp(*version),
            false => None,
        }
    }
}

impl<O: Send + Sync, I: Info> PartialOrd<Bundle> for Plugin<'_, O, I> {
    fn partial_cmp(&self, Bundle { id, version, .. }: &Bundle) -> Option<Ordering> {
        match self.info.bundle.id == *id {
            true => self.info.bundle.version.partial_cmp(version),
            false => None,
        }
    }
}

impl<O: Send + Sync, I: Info> PartialOrd<Depend> for Plugin<'_, O, I> {
    fn partial_cmp(
        &self,
        Depend {
            id: name, version, ..
        }: &Depend,
    ) -> Option<Ordering> {
        match self.info.bundle.id == *name {
            true => self.info.bundle.version.partial_cmp(version),
            false => None,
        }
    }
}

impl<O: Send + Sync, I: Info> Debug for Plugin<'_, O, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plugin")
            .field("id", &self.info.bundle.id)
            .field("version", &self.info.bundle.version)
            .field("format", &self.info.bundle.format)
            .field("path", &self.info.path)
            .field("is_load", &self.is_load)
            .field("depends", self.info.info.depends())
            .field("optional_depends", self.info.info.optional_depends())
            .finish()
    }
}
