use super::AppSwitch;
use crate::services::{
    api::{self, Session},
    locale,
};
use yew::{
    html,
    services::fetch::FetchTask,
    Callback,
    Component,
    ComponentLink,
    Html,
    ShouldRender,
};
use yew_router::{
    agent::{RouteAgentDispatcher, RouteRequest},
    route::Route,
};

#[derive(Debug)]
pub enum HomePageMsg {
    StartSession,
    SessionCreated(anyhow::Result<Session>),
}

#[derive(Debug)]
pub struct HomePage {
    link: ComponentLink<Self>,
    router_dispatcher: RouteAgentDispatcher<()>,
    session_state: SessionState,
}
impl Component for HomePage {
    type Message = HomePageMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            router_dispatcher: RouteAgentDispatcher::new(),
            session_state: SessionState::Idle,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use HomePageMsg::*;
        match msg {
            StartSession => {
                self.session_state.start(self.link.callback(SessionCreated));
                true
            }
            SessionCreated(resp) => {
                self.session_state
                    .handle_response(&mut self.router_dispatcher, resp);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| HomePageMsg::StartSession);
        html! {
            <>
                <button onclick=onclick>
                    { locale::get("create-session", None) }
                </button>
            </>
        }
    }
}

#[derive(Debug)]
enum SessionState {
    Idle,
    Loading(FetchTask),
    Failed(anyhow::Error),
}
impl SessionState {
    fn start(&mut self, callback: Callback<anyhow::Result<Session>>) {
        if matches!(self, Self::Loading(_)) {
            log::info!("session is already being created");
            return;
        }

        *self =
            Self::Loading(api::create_session(callback).expect("failed to create session request"))
    }

    fn handle_response(
        &mut self,
        router_dispatcher: &mut RouteAgentDispatcher<()>,
        resp: anyhow::Result<Session>,
    ) {
        match resp {
            Ok(session) => {
                let route = Route::from(AppSwitch::Sandbox(session.id));
                router_dispatcher.send(RouteRequest::ChangeRoute(route));
                *self = Self::Idle
            }
            Err(err) => {
                log::error!("error while creating session: {}", err);
                *self = Self::Failed(err);
            }
        }
    }
}
