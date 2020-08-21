use crate::{
    services::{
        api::{CompileResponse, Session, SessionRef},
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
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ActionBarProps {
    pub session: SessionRef,
    pub oncompile: Callback<CompileResponse>,
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
        use ActionBarMsg::*;
        match msg {
            Compile => self.state.compile(
                &self.props.session,
                self.link.callback(ActionBarMsg::CompileResponse),
            ),
            CompileResponse(resp) => {
                if let Some(resp) = self.state.handle_compile_response(resp) {
                    self.props.oncompile.emit(resp);
                }

                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let onclick_compile = self.link.callback(|_| ActionBarMsg::Compile);
        html! {
            <div class="action-bar">
                <button onclick=onclick_compile>{ locale::get("action_bar-compile", None) }</button>
            </div>
        }
    }
}

#[derive(Debug)]
enum ActionBarState {
    Idle,
    Compiling(FetchTask),
    Error(anyhow::Error),
}
impl ActionBarState {
    fn is_loading(&self) -> bool {
        matches!(self, Self::Compiling(_))
    }

    fn compile(
        &mut self,
        session: &Session,
        callback: Callback<anyhow::Result<CompileResponse>>,
    ) -> bool {
        if self.is_loading() {
            return false;
        }

        *self = Self::Compiling(
            session
                .compile(callback)
                .expect("failed to create compile request"),
        );
        true
    }

    fn handle_compile_response(
        &mut self,
        resp: anyhow::Result<CompileResponse>,
    ) -> Option<CompileResponse> {
        match resp {
            Ok(resp) => {
                *self = Self::Idle;
                Some(resp)
            }
            Err(err) => {
                *self = Self::Error(err);
                None
            }
        }
    }
}
