#[derive(thiserror::Error, Debug)]
pub enum RegisterPluginError {
    #[error("Does not contain config")]
    DoesNotContainConfig,
}
