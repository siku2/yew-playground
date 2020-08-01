use super::editor::Editor;
use crate::services::locale;
use yew::{html, services::fetch::FetchTask, Component, ComponentLink, Html, ShouldRender};

#[derive(Debug)]
pub enum PlaygroundMessage {
    RunCode,
}

pub struct Playground {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}
impl Component for Playground {
    type Message = PlaygroundMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_task: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let run_onclick = self.link.callback(|_| PlaygroundMessage::RunCode);

        html! {
            <div>
                <Editor />
                <div>
                    <button onclick=run_onclick>{ locale::get("playground-run", None) }</button>
                </div>
            </div>
        }
    }
}
