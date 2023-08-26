use std::{cmp::Ordering, fmt::Display, path::PathBuf};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::{utils::bundle::Bundle, Plugin};

pub struct PluginInfo<I: Info> {
    pub path: PathBuf,
    pub bundle: Bundle,
    pub info: I,
}

pub trait Info {
    fn depends(&self) -> &Vec<Depend>;
    fn optional_depends(&self) -> &Vec<Depend>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct StdInfo {
    pub depends: Vec<Depend>,
    pub optional_depends: Vec<Depend>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct Depend {
    pub id: String,
    pub version: Version,
}

impl StdInfo {
    pub const fn new() -> Self {
        Self {
            depends: vec![],
            optional_depends: vec![],
        }
    }
}

impl Info for StdInfo {
    fn depends(&self) -> &Vec<Depend> {
        &self.depends
    }

    fn optional_depends(&self) -> &Vec<Depend> {
        &self.optional_depends
    }
}

impl Display for StdInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Dependencies: {};{}Optional dependencies: {}",
            self.depends
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            f.alternate().then_some('\n').unwrap_or(' '),
            self.optional_depends
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Depend {
    pub const fn new(name: String, version: Version) -> Self {
        Self { id: name, version }
    }
}

impl Display for Depend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-v{}", self.id, self.version)
    }
}

impl<ID: AsRef<str>> PartialEq<(ID, &Version)> for Depend {
    fn eq(&self, (id, version): &(ID, &Version)) -> bool {
        self.id == id.as_ref() && self.version == **version
    }
}

impl PartialEq<Bundle> for Depend {
    fn eq(&self, Bundle { id, version, .. }: &Bundle) -> bool {
        self.id == *id && self.version == *version
    }
}

impl<O: Send + Sync, I: Info> PartialEq<Plugin<'_, O, I>> for Depend {
    fn eq(&self, other: &Plugin<'_, O, I>) -> bool {
        self.id == other.info.bundle.id && self.version == other.info.bundle.version
    }
}

impl<ID: AsRef<str>> PartialOrd<(ID, &Version)> for Depend {
    fn partial_cmp(&self, (id, version): &(ID, &Version)) -> Option<Ordering> {
        match self.id == *id.as_ref() {
            true => self.version.partial_cmp(*version),
            false => None,
        }
    }
}

impl PartialOrd<Bundle> for Depend {
    fn partial_cmp(&self, Bundle { id, version, .. }: &Bundle) -> Option<Ordering> {
        match self.id == *id {
            true => self.version.partial_cmp(version),
            false => None,
        }
    }
}

impl<O: Send + Sync, I: Info> PartialOrd<Plugin<'_, O, I>> for Depend {
    fn partial_cmp(&self, other: &Plugin<'_, O, I>) -> Option<Ordering> {
        match self.id == other.info.bundle.id {
            true => self.version.partial_cmp(&other.info.bundle.version),
            false => None,
        }
    }
}
