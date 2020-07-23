use super::Editor;
use crate::services::locale;
use yew::{html, Component, ComponentLink, ShouldRender};

#[derive(Clone, Debug)]
pub enum SandboxMessage {
    RunCode,
}

pub struct Sandbox {
    link: ComponentLink<Self>,
}
impl Component for Sandbox {
    type Message = SandboxMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let run_onclick = self.link.callback(|_| SandboxMessage::RunCode);

        html! {
            <div>
                <Editor />
                <div>
                    <button onclick=run_onclick>{ locale::get("sandbox-run", None) }</button>
                </div>
            </div>
        }
    }
}
