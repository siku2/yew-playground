use crate::{
    components::{
        browser::{Browser, Controller as BrowserController},
        console::{Console, ConsoleProps},
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
    console_props: ConsoleProps,
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
            console_props: ConsoleProps::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use SandboxPageMsg::*;
        match msg {
            Compiled(resp) => {
                self.browser_controller.reload();
                self.console_props = ConsoleProps {
                    stderr: resp.stderr,
                    stdout: resp.stdout,
                };
                true
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
            console_props,
            ..
        } = self;

        let oncompile = self.link.callback(SandboxPageMsg::Compiled);
        let console_props = console_props.clone();

        html! {
            <main>
                <Editor session=Rc::clone(session) oncompile=oncompile />
                <Console with console_props />
                <Browser session=Rc::clone(session) controller=browser_controller />
            </main>
        }
    }
}
