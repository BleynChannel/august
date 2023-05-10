use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub struct PluginInfo {
    pub id: String,
    pub depends: Vec<String>,
    pub optional_depends: Vec<String>,
}

impl PluginInfo {
    pub fn new(id: String) -> Self {
        Self {
            id,
            depends: Vec::new(),
            optional_depends: Vec::new(),
        }
    }
}