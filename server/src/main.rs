#![feature(decl_macro, hash_set_entry, never_type, proc_macro_hygiene)]

use janitor::Janitor;
use protocol::{CompileRequest, CompileResponse, SessionDetails};
use rocket::{response::NamedFile, State};
use rocket_contrib::{json::Json, serve::StaticFiles, uuid::Uuid as UuidParam};
use sandbox::Sandbox;
use std::path::PathBuf;

mod janitor;
mod sandbox;

#[rocket::post("/sandbox")]
fn api_sandbox_create(janitor: State<Janitor>) -> Result<Json<SessionDetails>, sandbox::Error> {
    // TODO create from template
    let sandbox = Sandbox::create()?;
    let session = janitor.create_session(sandbox);
    Ok(Json(SessionDetails {
        id: session.id.to_string(),
        // TODO actual public url
        public_url: format!("http://localhost:8000/proxy/{}", session.id),
    }))
}

#[rocket::get("/files/<sandbox>")]
fn api_sandbox_list_files(janitor: State<Janitor>, sandbox: UuidParam) -> Json<()> {
    todo!()
}

#[rocket::get("/files/<sandbox>/<path..>")]
fn api_sandbox_get_file(janitor: State<Janitor>, sandbox: UuidParam, path: PathBuf) -> Json<()> {
    todo!()
}
#[rocket::put("/files/<sandbox>/<path..>")]
fn api_sandbox_put_file(janitor: State<Janitor>, sandbox: UuidParam, path: PathBuf) -> Json<()> {
    todo!()
}

#[rocket::post("/compile", data = "<req>")]
fn api_sandbox_compile(
    janitor: State<Janitor>,
    req: Json<CompileRequest>,
) -> Result<Json<CompileResponse>, sandbox::Error> {
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
        .get_file_path(&path)
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
                api_sandbox_get_file,
                api_sandbox_put_file,
                api_sandbox_compile,
            ],
        )
        .mount("/proxy", rocket::routes![sandbox_get_file])
        // TODO make static location configurable
        .mount("/", StaticFiles::from("www"))
        .launch();
}
