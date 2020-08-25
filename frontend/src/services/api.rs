pub use protocol::{
    Channel,
    ClippyResponse,
    CompileResponse,
    Edition,
    FormatResponse,
    MacroExpandResponse,
    Mode,
    SandboxStructure,
    SessionDetails,
};
use protocol::{ClippyRequest, CompileRequest, FormatRequest, MacroExpandRequest};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, rc::Rc};
use yew::{
    format::{Json, Nothing, Text},
    services::fetch::{FetchService, FetchTask, Request, Response},
    Callback,
};

fn make_api_uri(path: impl Display) -> String {
    // TODO configurable api endpoint
    format!("/api{}", path)
}

pub fn create_session(callback: Callback<anyhow::Result<Session>>) -> anyhow::Result<FetchTask> {
    post_json(
        "/sandbox",
        &(),
        Callback::from(move |resp: anyhow::Result<SessionDetails>| {
            callback.emit(resp.map(Session::from))
        }),
    )
}

pub type SessionRef = Rc<Session>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Session {
    pub id: String,
    pub details: Option<SessionDetails>,
}
impl Session {
    pub fn new(id: String) -> Self {
        Self { id, details: None }
    }

    pub fn get_structure(
        &self,
        callback: Callback<anyhow::Result<SandboxStructure>>,
    ) -> anyhow::Result<FetchTask> {
        let req = Request::get(make_api_uri(format!("/{}/files", self.id)))
            .body(Nothing)
            .unwrap();

        perform_json_request(req, callback)
    }

    pub fn get_file(
        &self,
        path: &str,
        callback: Callback<anyhow::Result<String>>,
    ) -> anyhow::Result<FetchTask> {
        let req = Request::get(make_api_uri(format!("/{}/files/{}", self.id, path)))
            .body(Nothing)
            .unwrap();

        FetchService::fetch(
            req,
            Callback::from(move |response: Response<Text>| {
                let body = response.into_body();
                callback.emit(body)
            }),
        )
    }

    pub fn upload_file(
        &self,
        path: &str,
        content: String,
        callback: Callback<anyhow::Result<()>>,
    ) -> anyhow::Result<FetchTask> {
        let req = Request::put(make_api_uri(format!("/{}/files/{}", self.id, path)))
            .body(Ok(content))
            .unwrap();

        FetchService::fetch(
            req,
            Callback::from(move |response: Response<Text>| {
                let body = response.into_body().map(|_| ());
                callback.emit(body)
            }),
        )
    }

    pub fn compile(
        &self,
        callback: Callback<anyhow::Result<CompileResponse>>,
    ) -> anyhow::Result<FetchTask> {
        // TODO have these settings be stored on the server
        let body = CompileRequest {
            channel: Channel::Stable,
            mode: Mode::Debug,
            edition: None,
            backtrace: false,
        };

        post_json(format!("/{}/compile", self.id), &body, callback)
    }

    pub fn format(
        &self,
        callback: Callback<anyhow::Result<FormatResponse>>,
    ) -> anyhow::Result<FetchTask> {
        let body = FormatRequest { edition: None };

        post_json(format!("/{}/format", self.id), &body, callback)
    }

    pub fn clippy(
        &self,
        callback: Callback<anyhow::Result<ClippyResponse>>,
    ) -> anyhow::Result<FetchTask> {
        let body = ClippyRequest { edition: None };

        post_json(format!("/{}/clippy", self.id), &body, callback)
    }

    pub fn macro_expand(
        &self,
        callback: Callback<anyhow::Result<MacroExpandResponse>>,
    ) -> anyhow::Result<FetchTask> {
        let body = MacroExpandRequest { edition: None };

        post_json(format!("/{}/macro-expand", self.id), &body, callback)
    }
}

impl From<SessionDetails> for Session {
    fn from(details: SessionDetails) -> Self {
        Self {
            id: details.id.clone(),
            details: Some(details),
        }
    }
}

fn perform_json_request<ReqBody, RespBody>(
    req: Request<ReqBody>,
    callback: Callback<anyhow::Result<RespBody>>,
) -> anyhow::Result<FetchTask>
where
    ReqBody: Into<Text>,
    RespBody: 'static + for<'de> Deserialize<'de>,
{
    FetchService::fetch(
        req,
        Callback::from(move |response: Response<Json<anyhow::Result<RespBody>>>| {
            let body = response.into_body().0;
            callback.emit(body)
        }),
    )
}

fn post_json<Resp>(
    path: impl Display,
    body: &impl Serialize,
    callback: Callback<anyhow::Result<Resp>>,
) -> anyhow::Result<FetchTask>
where
    Resp: 'static + for<'de> serde::Deserialize<'de>,
{
    let req = Request::post(make_api_uri(path)).body(Json(body)).unwrap();
    perform_json_request(req, callback)
}
