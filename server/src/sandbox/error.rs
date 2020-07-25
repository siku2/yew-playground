use std::{io, string::FromUtf8Error, time::Duration};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unable to create temporary directory: {0}")]
    UnableToCreateTempDir(#[source] io::Error),
    #[error("unable to create output directory: {0}")]
    UnableToCreateOutputDir(#[source] io::Error),
    #[error("unable to set permissions for output directory: {0}")]
    UnableToSetOutputPermissions(#[source] io::Error),
    #[error("unable to create source file: {0}")]
    UnableToCreateSourceFile(#[source] io::Error),
    #[error("unable to set permissions for source file: {0}")]
    UnableToSetSourcePermissions(#[source] io::Error),

    #[error("unable to execute the compiler: {0}")]
    UnableToExecuteCompiler(#[source] io::Error),
    #[error("compiler execution took longer than {0}ms", timeout.as_millis())]
    CompilerExecutionTimedOut { timeout: Duration },

    #[error("unable to read output file: {0}")]
    UnableToReadOutput(#[source] io::Error),
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
