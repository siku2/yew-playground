#[derive(Debug, Clone)]
pub struct CompileResponse {
    pub success: bool,
    pub code: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone)]
pub struct FormatResponse {
    pub success: bool,
    pub code: String,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone)]
pub struct ClippyResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone)]
pub struct MacroExpansionResponse {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}
