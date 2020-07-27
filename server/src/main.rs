#![feature(decl_macro, hash_set_entry, never_type, proc_macro_hygiene)]

use janitor::{Janitor, SessionRef};
use protocol::{CompileRequest, CompileResponse, SessionDetails, SessionResponse};
use rocket::{
    http::Status,
    outcome::IntoOutcome,
    request::{self, FromRequest},
    response::NamedFile,
    Request,
    State,
};
use rocket_contrib::{json::Json, serve::StaticFiles, uuid::Uuid as UuidParam};
use sandbox::Sandbox;
use std::{path::PathBuf, str::FromStr};

mod janitor;
mod sandbox;

#[derive(Debug)]
enum SessionError {
    CookieMissing,
    CookieInvalid,
    NotFound,
}

#[derive(Debug)]
struct SessionIdGuard(UuidParam);
impl<'a, 'r> FromRequest<'a, 'r> for SessionIdGuard {
    type Error = SessionError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let value = cookies
            .get("session_id")
            .map(|cookie| cookie.value())
            .into_outcome((Status::BadRequest, SessionError::CookieMissing))?;

        UuidParam::from_str(value)
            .ok()
            .map(Self)
            .into_outcome((Status::BadRequest, SessionError::CookieInvalid))
    }
}

#[derive(Debug)]
struct SessionGuard(SessionRef);
impl SessionGuard {
    #[inline(always)]
    fn inner(&self) -> &SessionRef {
        &self.0
    }
}
impl<'a, 'r> FromRequest<'a, 'r> for SessionGuard {
    type Error = SessionError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let session_id = request.guard::<SessionIdGuard>()?;
        let janitor = request
            .guard::<State<Janitor>>()
            .succeeded()
            .or_forward(())?;
        janitor
            .inner()
            .get_session(&session_id.0)
            .map(Self)
            .into_outcome((Status::NotFound, SessionError::NotFound))
    }
}

#[rocket::post("/session")]
fn create_session(janitor: State<Janitor>) -> Result<Json<SessionDetails>, sandbox::Error> {
    let sandbox = Sandbox::create()?;
    let session = janitor.create_session(sandbox);
    Ok(Json(SessionDetails {
        id: session.id.to_string(),
    }))
}

#[rocket::get("/session")]
fn get_session(session: Option<SessionGuard>) -> Json<SessionResponse> {
    let details = session
        .as_ref()
        .map(SessionGuard::inner)
        .map(|session| SessionDetails {
            id: session.id.to_string(),
        });
    Json(SessionResponse { details })
}

#[rocket::post("/compile", data = "<req>")]
fn compile(
    session: SessionGuard,
    req: Json<CompileRequest>,
) -> Result<Json<CompileResponse>, sandbox::Error> {
    session.inner().sandbox.compile(&req.into_inner()).map(Json)
}

/// Returns a file from a sandbox
#[rocket::get("/<sandbox>/<path..>")]
fn get_sandbox_file(
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
        .mount("/api", rocket::routes![get_session, compile])
        .mount("/sandbox", rocket::routes![get_sandbox_file])
        // TODO make static location configurable
        .mount("/", StaticFiles::from("www"))
        .launch();
}
