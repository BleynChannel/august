use thiserror::Error;

#[derive(Error, Debug)]
pub enum StopLoaderError {
    #[error("Failed to unregister plugins `{0:?}`")]
    UnregisterPluginFailed(Vec<(String, UnregisterPluginError)>),
    #[error("Failed to unregister managers `{0:?}`")]
    UnregisterManagerFailed(Vec<(String, UnregisterManagerError)>),
}

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
pub enum LoadPluginError {
    #[error("The following dependencies could not be found: {0:?}")]
    NotFoundDependencies(Vec<String>),
    #[error("Dependency `{depend:?}` returned an error: {error:?}")]
    LoadDependency {
        depend: String,
        error: Box<LoadPluginError>,
    },
    #[error("Plugin load error by the manager")]
    LoadPluginByManager(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum UnloadPluginError {
    #[error("The plugin is dependent on plugin `{0}`")]
    DependentOnAnotherPlugin(String),
    #[error("Plugin unload error by the manager")]
    UnloadPluginByManager(#[from] anyhow::Error),
}
