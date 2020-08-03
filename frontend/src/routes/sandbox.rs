use crate::{
    components::{
        browser::{Browser, Controller as BrowserController},
        editor::Editor,
    },
    services::api::{Session, SessionRef},
    utils::NeqAssign,
};
use protocol::CompileResponse;
use std::rc::Rc;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
pub enum SandboxPageMsg {
    Compiled(CompileResponse),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SandboxPageProps {
    pub id: String,
}

#[derive(Debug)]
pub struct SandboxPage {
    props: SandboxPageProps,
    link: ComponentLink<Self>,
    session: SessionRef,
    browser_controller: BrowserController,
}
impl Component for SandboxPage {
    type Message = SandboxPageMsg;
    type Properties = SandboxPageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let session = Rc::new(Session::new(props.id.clone()));
        Self {
            props,
            link,
            session,
            browser_controller: BrowserController::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use SandboxPageMsg::*;
        match msg {
            Compiled(_resp) => {
                self.browser_controller.reload();
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let Self {
            session,
            browser_controller,
            ..
        } = self;

        let oncompile = self.link.callback(SandboxPageMsg::Compiled);

        html! {
            <main>
                <Editor session=Rc::clone(session) oncompile=oncompile />
                <Browser session=Rc::clone(session) controller=browser_controller />
            </main>
        }
    }
}
