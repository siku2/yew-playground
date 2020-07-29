use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Error {
    InternalError(String),

    SessionNotFound,

    SandboxFileNotFound,
}
