#![feature(decl_macro, hash_set_entry, never_type, proc_macro_hygiene)]

use janitor::{Janitor, SessionRef};
use protocol::{CompileRequest, CompileResponse, SandboxStructure, SessionDetails};
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
fn api_sandbox_create(janitor: State<Janitor>) -> Result<Json<SessionDetails>> {
    // TODO configurable template
    let sandbox = Sandbox::create_from_template(Path::new("template"))?;
    let session = janitor.create_session(sandbox);
    Ok(Json(SessionDetails {
        id: session.id.to_string(),
        // TODO actual public url
        public_url: format!("http://localhost:8000/proxy/{}", session.id),
    }))
}

fn get_session(janitor: &Janitor, id: &UuidParam) -> Result<SessionRef> {
    janitor
        .get_session(id)
        .ok_or_else(|| Error::from(protocol::Error::SessionNotFound))
}

#[rocket::get("/files/<sandbox>")]
fn api_sandbox_list_files(
    janitor: State<Janitor>,
    sandbox: UuidParam,
) -> Result<Json<SandboxStructure>> {
    let session = get_session(&janitor, &sandbox)?;
    let structure = session.sandbox.get_structure()?;
    Ok(Json(structure))
}

#[rocket::get("/files/<sandbox>/src/<path..>")]
fn api_sandbox_get_src_file(
    janitor: State<Janitor>,
    sandbox: UuidParam,
    path: PathBuf,
) -> Result<Content<NamedFile>> {
    let session = get_session(&janitor, &sandbox)?;
    let file = session
        .sandbox
        .get_src_path(&path)
        .and_then(|path| NamedFile::open(path).ok())
        .ok_or_else(|| Error::from(protocol::Error::SandboxFileNotFound))?;

    Ok(Content(ContentType::Plain, file))
}

#[rocket::put("/files/<sandbox>/src/<path..>", data = "<code>")]
fn api_sandbox_put_src_file(
    janitor: State<Janitor>,
    sandbox: UuidParam,
    path: PathBuf,
    code: String,
) -> Result<()> {
    let session = get_session(&janitor, &sandbox)?;
    session.sandbox.write_src_file(&path, &code)?;
    Ok(())
}

#[rocket::post("/compile", data = "<req>")]
fn api_sandbox_compile(
    janitor: State<Janitor>,
    req: Json<CompileRequest>,
) -> Result<Json<CompileResponse>> {
    todo!()
}

/// Returns a file from a sandbox
#[rocket::get("/<sandbox>/<path..>")]
fn sandbox_get_file(
    janitor: State<Janitor>,
    sandbox: UuidParam,
    path: PathBuf,
) -> Option<NamedFile> {
    // TODO return error 400 if the sandbox doesn't exist;
    let session = janitor.get_session(&*sandbox)?;

    session
        .sandbox
        .get_www_path(&path)
        .and_then(|path| NamedFile::open(path).ok())
}

fn main() {
    rocket::ignite()
        .manage(Janitor::default())
        .mount(
            "/api",
            rocket::routes![
                api_sandbox_create,
                api_sandbox_list_files,
                api_sandbox_get_src_file,
                api_sandbox_put_src_file,
                api_sandbox_compile,
            ],
        )
        .mount("/proxy", rocket::routes![sandbox_get_file])
        // TODO make static location configurable
        .mount("/", SPAStaticFiles::new("www"))
        .launch();
}
