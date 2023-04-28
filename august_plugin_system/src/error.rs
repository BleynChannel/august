use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegisterManagerError {
	#[error("Format `{0}` is already occupied")]
	AlreadyOccupiedFormat(String),
	#[error("Manager registration error by the manager")]
    RegisterManagerByManager(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum UnregisterManagerError {
	#[error("Not found manager with format `{0}`")]
	NotFound(String),
	#[error("Manager unregistration error by the manager")]
    UnregisterManagerByManager(#[from] anyhow::Error),
}

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
	#[error("A plugin with this name already exists")]
	AlreadyExistsName(String),
    #[error("Plugin registration error by the manager")]
    RegisterPluginByManager(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum UnregisterPluginError {
	#[error("Not found plugin")]
	NotFound,
	#[error("Plugin is loaded")]
	IsLoaded,
	#[error("The plugin has an unregistered manager")]
	HasUnregisteredManager,
	#[error("Plugin unregistration error by the manager")]
    UnregisterPluginByManager(#[from] anyhow::Error),
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
