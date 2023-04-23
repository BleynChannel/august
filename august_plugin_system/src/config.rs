use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	name: String,
    description: String,
    version: String,
    author: String,
    license: String,
}