#![feature(decl_macro, hash_set_entry, never_type, proc_macro_hygiene)]

use janitor::{Janitor, SessionRef};
use protocol::{CompileRequest, CompileResponse, SandboxStructure, SessionDetails, ToolVersions};
use response::Content;
use rocket::{
    http::{ContentType, Status},
    response::{self, NamedFile, Responder},
    Response,
    State,
};
use rocket_contrib::{json::Json, uuid::Uuid as UuidParam};
use sandbox::Sandbox;
use serve::SPAStaticFiles;
use std::path::{Path, PathBuf};

mod janitor;
mod sandbox;
mod serve;

#[derive(Debug)]
struct Error(Status, protocol::Error);
impl From<sandbox::Error> for Error {
    fn from(err: sandbox::Error) -> Self {
        log::error!("internal sandbox error: {:?}", err);
        Self::from(protocol::Error::InternalError(err.to_string()))
    }
}
impl From<protocol::Error> for Error {
    fn from(err: protocol::Error) -> Self {
        use protocol::Error::*;
        match err {
            InternalError(_) => Self(Status::InternalServerError, err),
            SessionNotFound | SandboxFileNotFound => Self(Status::NotFound, err),
        }
    }
}
impl<'r> Responder<'r> for Error {
    fn respond_to(self, request: &rocket::Request) -> response::Result<'r> {
        let Error(status, error) = self;
        Response::build()
            .merge(Json(error).respond_to(request)?)
            .status(status)
            .ok()
    }
}

type Result<T> = std::result::Result<T, Error>;

#[rocket::post("/sandbox")]
fn api_create_sandbox(janitor: State<Janitor>) -> Result<Json<SessionDetails>> {
    // TODO configurable template
    let sandbox = Sandbox::create_from_template(Path::new("template"))?;
    let session = janitor.create_session(sandbox);
    Ok(Json(SessionDetails {
        id: session.get_id_string(),
        // TODO actual public url
        public_url: format!("http://localhost:8000/proxy/{}/", session.get_id_string()),
    }))
}

fn get_session(janitor: &Janitor, id: &UuidParam) -> Result<SessionRef> {
    janitor
        .get_session(id)
        .ok_or_else(|| Error::from(protocol::Error::SessionNotFound))
}

#[rocket::get("/<sandbox>/tools")]
fn api_get_tool_versions(
    janitor: State<Janitor>,
    sandbox: UuidParam,
) -> Result<Json<ToolVersions>> {
    let session = get_session(&janitor, &sandbox)?;
    session
        .sandbox
        .get_tool_versions()
        .map(Json)
        .map_err(Error::from)
}

#[rocket::get("/<sandbox>/files")]
fn api_get_structure(
    janitor: State<Janitor>,
    sandbox: UuidParam,
) -> Result<Json<SandboxStructure>> {
    let session = get_session(&janitor, &sandbox)?;
    let structure = session.sandbox.get_structure()?;
    Ok(Json(structure))
}

#[rocket::get("/<sandbox>/files/<path..>")]
fn api_get_file(
    janitor: State<Janitor>,
    sandbox: UuidParam,
    path: PathBuf,
) -> Result<Content<NamedFile>> {
    let session = get_session(&janitor, &sandbox)?;
    let file = session
        .sandbox
        .get_file_path(&path)
        .ok()
        .and_then(|path| NamedFile::open(path).ok())
        .ok_or_else(|| Error::from(protocol::Error::SandboxFileNotFound))?;

    Ok(Content(ContentType::Plain, file))
}

#[rocket::put("/<sandbox>/files/<path..>", data = "<code>")]
fn api_upload_file(
    janitor: State<Janitor>,
    sandbox: UuidParam,
    path: PathBuf,
    code: String,
) -> Result<()> {
    let session = get_session(&janitor, &sandbox)?;
    session.sandbox.write_to_file(&path, &code)?;
    Ok(())
}

#[rocket::post("/<sandbox>/compile", data = "<req>")]
fn api_compile(
    janitor: State<Janitor>,
    sandbox: UuidParam,
    req: Json<CompileRequest>,
) -> Result<Json<CompileResponse>> {
    let session = get_session(&janitor, &sandbox)?;
    session
        .sandbox
        .compile(&*req)
        .map(Json)
        .map_err(Error::from)
}

#[rocket::get("/<sandbox>")]
fn sandbox_get_index(
    janitor: State<Janitor>,
    sandbox: UuidParam,
) -> std::result::Result<NamedFile, Status> {
    sandbox_get_file(janitor, sandbox, "index.html".into())
}

#[rocket::get("/<sandbox>/<path..>")]
fn sandbox_get_file(
    janitor: State<Janitor>,
    sandbox: UuidParam,
    path: PathBuf,
) -> std::result::Result<NamedFile, Status> {
    let session = get_session(&janitor, &sandbox).map_err(|_| Status::BadRequest)?;

    session
        .sandbox
        .get_serve_path(&path)
        .ok()
        .and_then(|path| NamedFile::open(path).ok())
        .ok_or(Status::NotFound)
}

fn main() {
    rocket::ignite()
        .manage(Janitor::default())
        .mount(
            "/api",
            rocket::routes![
                api_create_sandbox,
                api_get_tool_versions,
                api_get_structure,
                api_get_file,
                api_upload_file,
                api_compile,
            ],
        )
        .mount(
            "/proxy",
            rocket::routes![sandbox_get_index, sandbox_get_file],
        )
        // TODO make static location configurable
        .mount("/", SPAStaticFiles::new("www"))
        .launch();
}
