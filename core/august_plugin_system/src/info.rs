use std::{cmp::Ordering, fmt::Display, path::PathBuf};

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::{utils::bundle::Bundle, Plugin};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct PluginInfo {
    pub path: PathBuf,
    pub bundle: Bundle,
    pub info: Info,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
//TODO: Заменить на trait
pub struct Info {
    pub depends: Vec<Depend>,
    pub optional_depends: Vec<Depend>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct Depend {
    pub id: String,
    pub version: Version,
}

impl Info {
    pub const fn new() -> Self {
        Self {
            depends: vec![],
            optional_depends: vec![],
        }
    }
}

impl Depend {
    pub const fn new(name: String, version: Version) -> Self {
        Self { id: name, version }
    }
}

impl Display for Info {
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

impl<T: Send + Sync> PartialEq<Plugin<'_, T>> for Depend {
    fn eq(&self, other: &Plugin<'_, T>) -> bool {
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

impl<T: Send + Sync> PartialOrd<Plugin<'_, T>> for Depend {
    fn partial_cmp(&self, other: &Plugin<'_, T>) -> Option<Ordering> {
        match self.id == other.info.bundle.id {
            true => self.version.partial_cmp(&other.info.bundle.version),
            false => None,
        }
    }
}
