use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegisterPluginError {
    #[error("Not found plugin")]
    NotFound,
    #[error("Unpack error: {0}")]
    UnpackError(String),
    #[error("Unknown plugin manager for the format '{0}'")]
    UnknownManagerFormat(String),
    #[error("Does not contain config")]
    DoesNotContainConfig,
    #[error("Config reading error")]
    ConfigReading(#[from] std::io::Error),
    #[error("Config deserialization error")]
    ConfigDeserialization(#[from] toml::de::Error),
    #[error("Plugin registration error by the manager")]
    RegisterByManager(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum LoadPluginError<'a> {
    #[error("")]
    NotFoundDependencies(&'a [String]),
    #[error("")]
    LoadDependency((String, &'a LoadPluginError<'a>)),
    #[error("")]
    UnknownManagerFormat(String),
}
