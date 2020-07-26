#![feature(proc_macro_hygiene, decl_macro)]

use error::{Error, Result};
use protocol::{CompileRequest, CompileResponse};
use rocket::http::RawStr;
use rocket_contrib::{json::Json, serve::StaticFiles};
use sandbox::Sandbox;
use std::path::PathBuf;

mod error;
mod sandbox;

#[rocket::post("/compile", data = "<req>")]
fn compile(req: Json<CompileRequest>) -> Result<Json<CompileResponse>> {
    let sandbox = Sandbox::create().map_err(Error::SandboxCreation)?;
    sandbox
        .compile(&req.into_inner())
        .map_err(Error::Compilation)
        .map(CompileResponse::from)
        .map(Json)
}

#[rocket::get("/<sandbox>/<path..>")]
fn get_sandbox_file(sandbox: &RawStr, path: PathBuf) -> Result<String> {
    todo!("get ")
}

fn main() {
    rocket::ignite()
        .mount("/api", rocket::routes![compile])
        .mount("/sandbox", rocket::routes![get_sandbox_file])
        // TODO make static location configurable
        .mount("/", StaticFiles::from("www"))
        .launch();
}
