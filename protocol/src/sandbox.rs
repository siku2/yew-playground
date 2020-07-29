use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Channel {
    Stable,
    Nightly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Mode {
    Debug,
    Release,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Edition {
    Rust2018,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompileRequest {
    pub channel: Channel,
    pub mode: Mode,
    #[serde(default)]
    pub edition: Option<Edition>,
    pub backtrace: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompileResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormatRequest {
    #[serde(default)]
    pub edition: Option<Edition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormatResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClippyRequest {
    #[serde(default)]
    pub edition: Option<Edition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClippyResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MacroExpansionRequest {
    #[serde(default)]
    pub edition: Option<Edition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MacroExpansionResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}
