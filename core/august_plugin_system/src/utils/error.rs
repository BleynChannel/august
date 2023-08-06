use std::{
    error::Error as StdError,
    fmt::{Debug, Display},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StopLoaderError {
    #[error("Failed to unregister plugins `{0:?}`")]
    UnregisterPluginFailed(Vec<(String, UnregisterPluginError)>),
    #[error("Failed to unregister managers `{0:?}`")]
    UnregisterManagerFailed(Vec<UnregisterManagerError>),
}

#[derive(Error, Debug)]
pub enum RegisterManagerError {
    #[error("Format `{0}` is already occupied")]
    AlreadyOccupiedFormat(String),
    #[error("Manager registration error by the manager")]
    RegisterManagerByManager(#[from] Box<dyn StdError>),
}

#[derive(Error, Debug)]
pub enum UnregisterManagerError {
    #[error("Not found manager")]
    NotFound,
    #[error("Manager unregistration error by the manager")]
    UnregisterManagerByManager(#[from] Box<dyn StdError>),
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
    RegisterPluginByManager(#[from] Box<dyn StdError>),
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
    UnregisterPluginByManager(#[from] Box<dyn StdError>),
}

#[derive(Error, Debug)]
pub enum LoadPluginError {
    #[error("Not found plugin")]
    NotFound,
    #[error("The following dependencies could not be found: {0:?}")]
    NotFoundDependencies(Vec<String>),
    #[error("Dependency `{depend:?}` returned an error: {error:?}")]
    LoadDependency {
        depend: String,
        error: Box<LoadPluginError>,
    },
    #[error("Plugin load error by the manager")]
    LoadPluginByManager(#[from] Box<dyn StdError>),
    #[error("Requests not found: {0:?}")]
    RequestsNotFound(Vec<String>),
}

#[derive(Error, Debug)]
pub enum UnloadPluginError {
    #[error("Not found plugin")]
    NotFound,
    #[error("The plugin is dependent on plugin `{0}`")]
    DependentOnAnotherPlugin(String),
    #[error("Plugin unload error by the manager")]
    UnloadPluginByManager(#[from] Box<dyn StdError>),
}

#[derive(Error, Debug)]
pub enum RegisterRequestError {
    #[error("Function not found")]
    NotFound,
    #[error("The arguments are set incorrectly")]
    ArgumentsIncorrectly,
}

#[derive(Error, Debug)]
pub enum PluginCallRequest {
    #[error("Request not found")]
    NotFound,
}

pub type ManagerResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct ParseVariableError {
    ty: &'static str,
}

impl ParseVariableError {
    pub fn new(ty: &'static str) -> Self {
        Self { ty }
    }
}

impl Display for ParseVariableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "data cannot be converted to this type `{}`", self.ty)
    }
}

impl StdError for ParseVariableError {}
