use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Plugin {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub license: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub plugin: Plugin,
}
