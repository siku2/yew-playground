use fluent::{FluentArgs, FluentBundle, FluentError, FluentMessage, FluentResource};
use fluent_syntax::parser::ParserError;
use std::{
    cell::{BorrowError, RefCell},
    str::FromStr,
};
use unic_langid::LanguageIdentifier;
use yew::{
    format::{Nothing, Text},
    services::fetch::{FetchService, FetchTask, Request, Response},
    Callback,
};

thread_local! {
    static BUNDLE: RefCell<Option<FluentBundle<FluentResource>>> = RefCell::new(None);
}

/// Get the currently loaded language.
pub fn loaded_language() -> Option<LanguageIdentifier> {
    BUNDLE.with(|bundle| {
        bundle
            .borrow()
            .as_ref()
            .and_then(|bundle| bundle.locales.get(0).cloned())
    })
}

fn load_fluent_resource(lang_id: &LanguageIdentifier, resource: FluentResource) {
    let locales = vec![lang_id];

    let mut new_bundle = FluentBundle::new(locales);
    new_bundle.add_resource(resource).unwrap();

    BUNDLE.with(|bundle| bundle.borrow_mut().replace(new_bundle));
}

#[derive(Debug)]
#[must_use = "loading is aborted as soon as the task is dropped"]
pub struct LoadBundleTask(FetchTask);

/// Load the bundle for the given language.
pub fn load_bundle(
    lang: &str,
    callback: Callback<anyhow::Result<()>>,
) -> anyhow::Result<LoadBundleTask> {
    let lang_id = LanguageIdentifier::from_str(lang)?;

    let fetch_task = fetch_fluent_resource(
        &lang_id.to_string(),
        Callback::from(move |resp: Response<FluentFetchResult>| {
            let resource_res = resp.into_body().into();

            let res = match resource_res {
                Ok(resource) => {
                    load_fluent_resource(&lang_id, resource);
                    Ok(())
                }
                Err(err) => Err(err),
            };
            callback.emit(res);
        }),
    )?;

    Ok(LoadBundleTask(fetch_task))
}

pub fn load_default_bundle(
    callback: Callback<anyhow::Result<()>>,
) -> anyhow::Result<LoadBundleTask> {
    // TODO make language configurable and try browser language if not set
    load_bundle("en-GB", callback)
}

#[derive(Debug, thiserror::Error)]
pub enum LocaleError {
    #[error(transparent)]
    BorrowError(#[from] BorrowError),
    #[error("no locale bundle is currently loaded")]
    NoBundle,
    #[error("no message with id `{0}`")]
    MessageNotFound(String),
    #[error("message `{0}` has no value")]
    NoValue(String),
    #[error("formatting message `{0}` failed: {1:?}")]
    FormatError(String, Vec<FluentError>),
}

fn with_message<T>(
    id: &str,
    f: impl FnOnce(&FluentBundle<FluentResource>, FluentMessage) -> T,
) -> Result<T, LocaleError> {
    BUNDLE.with(|bundle| {
        let bundle_container = bundle.try_borrow()?;
        let bundle = bundle_container.as_ref().ok_or(LocaleError::NoBundle)?;
        let message = bundle
            .get_message(id)
            .ok_or_else(|| LocaleError::MessageNotFound(id.to_string()))?;
        Ok(f(bundle, message))
    })
}

/// Get the message with the given id.
pub fn try_get(id: &str, args: Option<&FluentArgs>) -> Result<String, LocaleError> {
    with_message(id, |bundle, msg| {
        let pattern = msg
            .value
            .ok_or_else(|| LocaleError::NoValue(id.to_string()))?;
        let mut errors = Vec::new();
        let text = bundle
            .format_pattern(pattern, args, &mut errors)
            .into_owned();

        if errors.is_empty() {
            Ok(text)
        } else {
            Err(LocaleError::FormatError(id.to_string(), errors))
        }
    })?
}

/// Get the message with the given id.
/// This uses `try_get` under the hood but handles the error by returning the
/// `id`.
pub fn get(id: &str, args: Option<&FluentArgs>) -> String {
    match try_get(id, args) {
        Ok(text) => text,
        Err(err) => {
            log::error!("{}", err);
            id.to_string()
        }
    }
}

#[derive(Debug)]
#[must_use]
enum FluentFetchResult {
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

/// Fetch the `FluentResource` for the given language.
fn fetch_fluent_resource(
    lang: &str,
    callback: Callback<Response<FluentFetchResult>>,
) -> anyhow::Result<FetchTask> {
    let req = Request::get(format!("/locale/{}.ftl", lang))
        .body(Nothing)
        .unwrap();
    FetchService::fetch(req, callback)
}
