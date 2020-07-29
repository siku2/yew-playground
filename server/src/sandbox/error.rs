use std::{io, path::PathBuf, string::FromUtf8Error, time::Duration};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unable to create temporary directory: {0}")]
    UnableToPrepareDir(#[source] io::Error),
    #[error("unable to set permissions: {0}")]
    UnableToSetPermissions(#[source] io::Error),
    #[error("unable to read file: {0}")]
    UnableToReadFile(#[source] io::Error),
    #[error("unable to write file: {0}")]
    UnableToWriteFile(#[source] io::Error),

    #[error("path is invalid: {0}")]
    InvalidPath(PathBuf),
    #[error("sandbox directory is corrupted")]
    CorruptSandboxDir,

    #[error("unable to execute the compiler: {0}")]
    UnableToExecuteCompiler(#[source] io::Error),
    #[error("compiler execution took longer than {0}ms", timeout.as_millis())]
    CompilerExecutionTimedOut { timeout: Duration },

    #[error("Unable to read crate information: {0}")]
    UnableToParseCrateInformation(#[source] serde_json::Error),
    #[error("output was not valid UTF-8: {0}")]
    OutputNotUtf8(#[source] FromUtf8Error),

    #[error("output was missing")]
    OutputMissing,
    #[error("release was missing from the version output")]
    VersionReleaseMissing,
    #[error("commit hash was missing from the version output")]
    VersionHashMissing,
    #[error("commit date was missing from the version output")]
    VersionDateMissing,
}
