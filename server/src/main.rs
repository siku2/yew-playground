#![feature(proc_macro_hygiene, decl_macro)]

use error::{Error, Result};
use rocket_contrib::json::Json;
use sandbox::{request, Channel, Edition, Mode, Sandbox};
use serde::Deserialize;
use std::convert::TryInto;

mod error;
mod sandbox;

#[derive(Debug, Clone, Deserialize)]
struct CompileRequest {
    channel: String,
    mode: String,
    #[serde(default)]
    edition: String,
    #[serde(rename = "crateType")]
    crate_type: String,
    #[serde(default)]
    backtrace: bool,
    code: String,
}
impl TryInto<request::CompileRequest> for CompileRequest {
    type Error = Error;

    fn try_into(self) -> Result<request::CompileRequest> {
        Ok(request::CompileRequest {
            channel: parse_channel(&self.channel)?,
            mode: parse_mode(&self.mode)?,
            edition: parse_edition(&self.edition)?,
            backtrace: self.backtrace,
            code: self.code,
        })
    }
}

#[rocket::post("/compile", format = "json", data = "<req>")]
fn compile(req: Json<CompileRequest>) -> Result<String> {
    let sandbox = Sandbox::create().map_err(Error::SandboxCreation)?;
    sandbox
        .compile(&req.into_inner().try_into()?)
        .map_err(Error::Compilation)?;
    Ok(format!("ok"))
}

fn main() {
    rocket::ignite()
        .mount("/", rocket::routes![compile])
        .launch();
}

fn parse_channel(s: &str) -> Result<Channel> {
    Ok(match s {
        "stable" => Channel::Stable,
        "beta" => Channel::Beta,
        "nightly" => Channel::Nightly,
        value => return Err(Error::InvalidChannel(value.to_owned())),
    })
}

fn parse_mode(s: &str) -> Result<Mode> {
    Ok(match s {
        "debug" => Mode::Debug,
        "release" => Mode::Release,
        value => return Err(Error::InvalidMode(value.to_owned())),
    })
}

fn parse_edition(s: &str) -> Result<Option<Edition>> {
    Ok(match s {
        "" => None,
        "2015" => Some(Edition::Rust2015),
        "2018" => Some(Edition::Rust2018),
        value => return Err(Error::InvalidEdition(value.to_owned())),
    })
}
