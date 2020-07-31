pub use protocol::{Channel, CompileRequest, CompileResponse, Edition, Mode, SessionDetails};
use serde::Serialize;
use yew::{
    format::{Json, Nothing, Text},
    services::fetch::{FetchService, FetchTask, Request, Response},
    Callback,
};

fn make_api_uri(path: &'static str) -> String {
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

pub struct Session {
    pub details: SessionDetails,
}
impl Session {
    pub fn compile(
        callback: Callback<anyhow::Result<CompileResponse>>,
    ) -> anyhow::Result<FetchTask> {
        let body = CompileRequest {
            channel: Channel::Stable,
            mode: Mode::Debug,
            edition: None,
            backtrace: false,
        };

        post_json("/compile", &body, callback)
    }
}

impl From<SessionDetails> for Session {
    fn from(details: SessionDetails) -> Self {
        Self { details }
    }
}

fn post_json<Resp>(
    path: &'static str,
    body: &impl Serialize,
    callback: Callback<anyhow::Result<Resp>>,
) -> anyhow::Result<FetchTask>
where
    Resp: 'static + for<'de> serde::Deserialize<'de>,
{
    let req = Request::post(make_api_uri(path)).body(Json(body)).unwrap();

    FetchService::fetch(
        req,
        Callback::from(move |response: Response<Json<anyhow::Result<Resp>>>| {
            let body = response.into_body().0;
            callback.emit(body)
        }),
    )
}
