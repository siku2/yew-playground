use crate::sandbox;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sandbox creation failed: {0}")]
    SandboxCreation(#[source] sandbox::Error),
    #[error("compilation operation failed: {0}")]
    Compilation(#[source] sandbox::Error),
    #[error("execution operation failed: {0}")]
    Execution(#[source] sandbox::Error),
    #[error("evaluation operation failed: {0}")]
    Evaluation(#[source] sandbox::Error),
    #[error("linting operation failed: {0}")]
    Linting(#[source] sandbox::Error),
    #[error("expansion operation failed: {0}")]
    Expansion(#[source] sandbox::Error),
    #[error("formatting operation failed: {0}")]
    Formatting(#[source] sandbox::Error),
    #[error("interpreting operation failed: {0}")]
    Interpreting(#[source] sandbox::Error),
    #[error("caching operation failed: {0}")]
    Caching(#[source] sandbox::Error),

    #[error("unable to serialize response: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("{0:?} is not a valid target")]
    InvalidTarget(String),
    #[error("{0:?} is not a valid assembly flavor")]
    InvalidAssemblyFlavor(String),
    #[error("{0:?} is not a valid demangle option")]
    InvalidDemangleAssembly(String),
    #[error("{0:?} is not a valid assembly processing option")]
    InvalidProcessAssembly(String),
    #[error("{0:?} is not a valid channel")]
    InvalidChannel(String),
    #[error("{0:?} is not a valid mode")]
    InvalidMode(String),
    #[error("{0:?} is not a valid edition")]
    InvalidEdition(String),
    #[error("{0:?} is not a valid crate type")]
    InvalidCrateType(String),
    #[error("no request was provided")]
    RequestMissing,
    #[error("cache has been poisoned")]
    CachePoisoned,
}
