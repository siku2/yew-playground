use rocket::{handler, http::Status, response::NamedFile, Data, Handler, Outcome, Request, Route};
use rocket_contrib::serve;
use serve::{Options, StaticFiles};
use std::path::{Path, PathBuf};

/// Modified `StaticFiles` handler to return the index file when appropriate.
#[derive(Clone)]
pub struct SPAStaticFiles {
    index_path: PathBuf,
    inner: StaticFiles,
}
impl SPAStaticFiles {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let index_path = path.join("index.html");
        let inner = StaticFiles::new(path, Options::Index | Options::NormalizeDirs);
        Self { index_path, inner }
    }
}
impl Into<Vec<Route>> for SPAStaticFiles {
    fn into(self) -> Vec<Route> {
        let handler = Box::new(self.clone());
        let routes: Vec<Route> = self.inner.into();
        routes
            .into_iter()
            .map(|mut route| {
                route.handler = handler.clone();
                route
            })
            .collect()
    }
}
impl Handler for SPAStaticFiles {
    fn handle<'r>(&self, request: &'r Request, data: Data) -> handler::Outcome<'r> {
        match self.inner.handle(request, data) {
            failure @ Outcome::Failure(Status::NotFound) => {
                let path = match request.uri().segments().into_path_buf(false) {
                    Ok(v) => v,
                    Err(_) => return failure,
                };
                let ext_ok = if let Some(ext) = path.extension() {
                    ext == "html"
                } else {
                    true
                };

                if ext_ok {
                    Outcome::from(request, NamedFile::open(&self.index_path).ok())
                } else {
                    failure
                }
            }
            outcome => outcome,
        }
    }
}
