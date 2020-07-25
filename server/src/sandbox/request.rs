use super::{Channel, Edition, Mode};

pub trait EditionRequest {
    fn edition(&self) -> Option<Edition>;
}
impl<R: EditionRequest> EditionRequest for &'_ R {
    fn edition(&self) -> Option<Edition> {
        (*self).edition()
    }
}

pub trait BacktraceRequest {
    fn backtrace(&self) -> bool;
}
impl<R: BacktraceRequest> BacktraceRequest for &'_ R {
    fn backtrace(&self) -> bool {
        (*self).backtrace()
    }
}

#[derive(Debug, Clone)]
pub struct CompileRequest {
    pub channel: Channel,
    pub mode: Mode,
    pub edition: Option<Edition>,
    pub tests: bool,
    pub backtrace: bool,
    pub code: String,
}
impl EditionRequest for CompileRequest {
    fn edition(&self) -> Option<Edition> {
        self.edition
    }
}
impl BacktraceRequest for CompileRequest {
    fn backtrace(&self) -> bool {
        self.backtrace
    }
}

#[derive(Debug, Clone)]
pub struct FormatRequest {
    pub code: String,
    pub edition: Option<Edition>,
}
impl EditionRequest for FormatRequest {
    fn edition(&self) -> Option<Edition> {
        self.edition
    }
}

#[derive(Debug, Clone)]
pub struct ClippyRequest {
    pub code: String,
    pub edition: Option<Edition>,
}
impl EditionRequest for ClippyRequest {
    fn edition(&self) -> Option<Edition> {
        self.edition
    }
}

#[derive(Debug, Clone)]
pub struct MacroExpansionRequest {
    pub code: String,
    pub edition: Option<Edition>,
}
impl EditionRequest for MacroExpansionRequest {
    fn edition(&self) -> Option<Edition> {
        self.edition
    }
}
