use std::path::PathBuf;

use crate::utils::bundle::Bundle;

pub struct RegisterPluginContext<'a> {
    pub path: &'a PathBuf,
    pub bundle: &'a Bundle,
}
