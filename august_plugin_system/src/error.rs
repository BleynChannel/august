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
	#[error("Not found manager")]
	NotFound,
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
    #[error("Plugin registration error by the manager")]
    RegisterPluginByManager(#[from] anyhow::Error),
	#[error("A plugin with this ID already exists")]
	AlreadyExistsID(String),
}

#[derive(Error, Debug)]
pub enum UnregisterPluginError {
	#[error("Not found plugin")]
	NotFound,
	#[error("Plugin unload error")]
	UnloadError(#[from] UnloadPluginError),
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
}

#[derive(Error, Debug)]
pub enum UnloadPluginError {
    #[error("")]
    NotFoundDependencies,
}
