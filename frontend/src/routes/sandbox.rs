use crate::{
    components::{
        action_bar::ActionBarCallbacks,
        browser::{Browser, Controller as BrowserController},
        console::{Console, ConsoleProps},
        editor::Editor,
    },
    services::api::{
        ClippyResponse,
        CompileResponse,
        FormatResponse,
        MacroExpandResponse,
        Session,
        SessionRef,
    },
    utils::NeqAssign,
};
use std::rc::Rc;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
pub enum SandboxPageMsg {
    ReloadBrowser,
    DisplayOutput { stdout: String, stderr: String },
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
    action_bar_callbacks: ActionBarCallbacks,
}
impl Component for SandboxPage {
    type Message = SandboxPageMsg;
    type Properties = SandboxPageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let action_bar_callbacks = build_action_bar_callbacks(&link);

        let session = Rc::new(Session::new(props.id.clone()));
        Self {
            props,
            link,
            session,
            browser_controller: BrowserController::default(),
            console_props: ConsoleProps::default(),
            action_bar_callbacks,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use SandboxPageMsg::*;
        match msg {
            ReloadBrowser => {
                self.browser_controller.reload();
                false
            }
            DisplayOutput { stdout, stderr } => {
                self.console_props = ConsoleProps { stderr, stdout };
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
            action_bar_callbacks,
            ..
        } = self;

        let console_props = console_props.clone();

        html! {
            <main>
                <Editor session=Rc::clone(session) action_bar_callbacks=action_bar_callbacks.clone() />
                <Console with console_props />
                <Browser session=Rc::clone(session) controller=browser_controller />
            </main>
        }
    }
}

fn build_action_bar_callbacks(link: &ComponentLink<SandboxPage>) -> ActionBarCallbacks {
    use SandboxPageMsg::*;
    ActionBarCallbacks {
        compile: link.batch_callback(|res: CompileResponse| {
            vec![
                ReloadBrowser,
                DisplayOutput {
                    stdout: res.stdout,
                    stderr: res.stderr,
                },
            ]
        }),
        format: link.callback(|res: FormatResponse| DisplayOutput {
            stdout: res.stdout,
            stderr: res.stderr,
        }),
        clippy: link.callback(|res: ClippyResponse| DisplayOutput {
            stdout: res.stdout,
            stderr: res.stderr,
        }),
        // TODO display the result in a new tab instead of the console
        macro_expand: link.callback(|res: MacroExpandResponse| DisplayOutput {
            stdout: res.stdout,
            stderr: res.stderr,
        }),
    }
}
