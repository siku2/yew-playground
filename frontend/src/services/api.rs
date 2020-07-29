pub use protocol::{Channel, CompileRequest, CompileResponse, Edition, Mode};
use yew::{
    format::Json,
    services::fetch::{FetchService, FetchTask, Request, Response},
    Callback,
};

pub fn compile_with_request(
    request: &CompileRequest,
    callback: Callback<anyhow::Result<CompileResponse>>,
) -> anyhow::Result<FetchTask> {
    // TODO configurable api endpoint
    let req = Request::post("/api/compile").body(Json(request)).unwrap();
    FetchService::fetch(
        req,
        Callback::from(
            move |response: Response<Json<anyhow::Result<CompileResponse>>>| {
                callback.emit(response.into_body().0)
            },
        ),
    )
}
