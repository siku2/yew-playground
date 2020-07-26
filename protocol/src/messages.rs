use super::{Channel, Edition, Mode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompileRequest {
    pub channel: Channel,
    pub mode: Mode,
    #[serde(default)]
    pub edition: Option<Edition>,
    pub backtrace: bool,
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompileResponse {
    pub success: bool,
    pub code: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormatRequest {
    pub code: String,
    #[serde(default)]
    pub edition: Option<Edition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormatResponse {
    pub success: bool,
    pub code: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClippyRequest {
    pub code: String,
    #[serde(default)]
    pub edition: Option<Edition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClippyResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MacroExpansionResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MacroExpansionRequest {
    pub code: String,
    #[serde(default)]
    pub edition: Option<Edition>,
}
