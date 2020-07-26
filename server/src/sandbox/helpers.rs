use super::{Error, Result};
use protocol::{
    Channel,
    ClippyRequest,
    CompileRequest,
    Edition,
    FormatRequest,
    MacroExpansionRequest,
};
use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read},
    path::Path,
};

pub fn string_from_utf8_vec(v: Vec<u8>) -> Result<String> {
    String::from_utf8(v).map_err(Error::OutputNotUtf8)
}

pub fn read_file_to_string(path: &Path) -> Result<Option<String>> {
    let f = match File::open(path) {
        Ok(f) => f,
        Err(ref e) if e.kind() == ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(Error::UnableToReadOutput(e)),
    };
    let mut f = BufReader::new(f);
    let mut s = String::new();
    f.read_to_string(&mut s)
        .map_err(Error::UnableToReadOutput)?;
    Ok(Some(s))
}

pub fn container_name_for_channel(channel: Channel) -> &'static str {
    use Channel::*;

    match channel {
        Stable => "compiler-stable",
        Nightly => "compiler-nightly",
    }
}

pub fn cargo_ident_for_edition(edition: Edition) -> &'static str {
    use Edition::*;

    match edition {
        Rust2018 => "2018",
    }
}

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

impl EditionRequest for FormatRequest {
    fn edition(&self) -> Option<Edition> {
        self.edition
    }
}

impl EditionRequest for ClippyRequest {
    fn edition(&self) -> Option<Edition> {
        self.edition
    }
}

impl EditionRequest for MacroExpansionRequest {
    fn edition(&self) -> Option<Edition> {
        self.edition
    }
}
