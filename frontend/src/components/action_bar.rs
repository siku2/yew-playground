use crate::{
    services::{
        api::{
            ClippyResponse,
            CompileResponse,
            FormatResponse,
            MacroExpandResponse,
            Session,
            SessionRef,
        },
        locale,
    },
    utils::NeqAssign,
};
use yew::{
    html,
    services::fetch::FetchTask,
    Callback,
    Component,
    ComponentLink,
    Html,
    Properties,
    ShouldRender,
};

#[derive(Debug)]
pub enum ActionBarMsg {
    Compile,
    CompileResponse(anyhow::Result<CompileResponse>),
    Format,
    FormatResponse(anyhow::Result<FormatResponse>),
    Clippy,
    ClippyResponse(anyhow::Result<ClippyResponse>),
    MacroExpand,
    MacroExpandResponse(anyhow::Result<MacroExpandResponse>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ActionBarCallbacks {
    pub compile: Callback<CompileResponse>,
    pub format: Callback<FormatResponse>,
    pub clippy: Callback<ClippyResponse>,
    pub macro_expand: Callback<MacroExpandResponse>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ActionBarProps {
    pub session: SessionRef,
    pub callbacks: ActionBarCallbacks,
}

#[derive(Debug)]
pub struct ActionBar {
    props: ActionBarProps,
    link: ComponentLink<Self>,
    state: ActionBarState,
}
impl Component for ActionBar {
    type Message = ActionBarMsg;
    type Properties = ActionBarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            state: ActionBarState::Idle,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let Self {
            props: ActionBarProps { session, callbacks },
            link,
            state,
        } = self;

        use ActionBarMsg::*;
        match msg {
            Compile => state.compile(&session, link.callback(ActionBarMsg::CompileResponse)),
            CompileResponse(resp) => {
                if let Some(resp) = state.handle_response(resp) {
                    callbacks.compile.emit(resp);
                }

                true
            }
            Format => state.format(&session, link.callback(ActionBarMsg::FormatResponse)),
            FormatResponse(resp) => {
                if let Some(resp) = state.handle_response(resp) {
                    callbacks.format.emit(resp);
                }
                true
            }
            Clippy => state.clippy(&session, link.callback(ActionBarMsg::ClippyResponse)),
            ClippyResponse(resp) => {
                if let Some(resp) = state.handle_response(resp) {
                    callbacks.clippy.emit(resp);
                }
                true
            }
            MacroExpand => {
                state.macro_expand(&session, link.callback(ActionBarMsg::MacroExpandResponse))
            }
            MacroExpandResponse(resp) => {
                if let Some(resp) = state.handle_response(resp) {
                    callbacks.macro_expand.emit(resp);
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let link = &self.link;
        html! {
            <div>
                <button onclick=link.callback(|_| ActionBarMsg::Compile)>
                    { locale::get("action_bar-compile", None) }
                </button>
                <button onclick=link.callback(|_| ActionBarMsg::Format)>
                    { locale::get("action_bar-format", None) }
                </button>
                <button onclick=link.callback(|_| ActionBarMsg::Clippy)>
                    { locale::get("action_bar-clippy", None) }
                </button>
                <button onclick=link.callback(|_| ActionBarMsg::MacroExpand)>
                    { locale::get("action_bar-macro_expand", None) }
                </button>
            </div>
        }
    }
}

#[derive(Debug)]
enum ActionBarState {
    Idle,
    Waiting(FetchTask),
    Error(anyhow::Error),
}
impl ActionBarState {
    fn is_loading(&self) -> bool {
        matches!(self, Self::Waiting(_))
    }

    fn compile(
        &mut self,
        session: &Session,
        callback: Callback<anyhow::Result<CompileResponse>>,
    ) -> bool {
        if self.is_loading() {
            return false;
        }
        *self = Self::Waiting(
            session
                .compile(callback)
                .expect("failed to create compile request"),
        );
        true
    }

    fn format(
        &mut self,
        session: &Session,
        callback: Callback<anyhow::Result<FormatResponse>>,
    ) -> bool {
        if self.is_loading() {
            return false;
        }
        *self = Self::Waiting(
            session
                .format(callback)
                .expect("failed to create format request"),
        );
        true
    }

    fn clippy(
        &mut self,
        session: &Session,
        callback: Callback<anyhow::Result<ClippyResponse>>,
    ) -> bool {
        if self.is_loading() {
            return false;
        }
        *self = Self::Waiting(
            session
                .clippy(callback)
                .expect("failed to create clippy request"),
        );
        true
    }

    fn macro_expand(
        &mut self,
        session: &Session,
        callback: Callback<anyhow::Result<MacroExpandResponse>>,
    ) -> bool {
        if self.is_loading() {
            return false;
        }
        *self = Self::Waiting(
            session
                .macro_expand(callback)
                .expect("failed to create macro-expand request"),
        );
        true
    }

    fn handle_response<T>(&mut self, resp: anyhow::Result<T>) -> Option<T> {
        match resp {
            Ok(res) => {
                *self = Self::Idle;
                Some(res)
            }
            Err(err) => {
                *self = Self::Error(err);
                None
            }
        }
    }
}
