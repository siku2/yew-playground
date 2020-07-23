use fluent::FluentResource;
use fluent_syntax::parser::ParserError;
use http::{Request, Response};
use yew::{
    format::{Nothing, Text},
    services::fetch::{FetchService, FetchTask},
    Callback,
};

#[derive(Debug)]
#[must_use]
pub enum FluentFetchResult {
    Ok(FluentResource),
    FetchError(anyhow::Error),
    ParseError(Vec<ParserError>),
}
impl From<Text> for FluentFetchResult {
    fn from(value: Text) -> Self {
        match value {
            Ok(text) => match FluentResource::try_new(text) {
                Ok(res) => Self::Ok(res),
                Err((_, err)) => Self::ParseError(err),
            },
            Err(err) => Self::FetchError(err),
        }
    }
}
impl Into<Result<FluentResource, anyhow::Error>> for FluentFetchResult {
    fn into(self) -> Result<FluentResource, anyhow::Error> {
        match self {
            Self::Ok(v) => Ok(v),
            Self::FetchError(err) => Err(err),
            Self::ParseError(err) => Err(anyhow::anyhow!("failed to parse fluent file: {:?}", err)),
        }
    }
}

pub type Task = FetchTask;

/// Fetch the `FluentResource` for the given language.
pub fn fetch(lang: &str, callback: Callback<Response<FluentFetchResult>>) -> anyhow::Result<Task> {
    let req = Request::get(format!("/locale/{}.ftl", lang))
        .body(Nothing)
        .unwrap();
    FetchService::fetch(req, callback)
}
