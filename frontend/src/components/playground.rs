use super::Editor;
use crate::services::locale;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[derive(Clone, Debug)]
pub enum PlaygroundMessage {
    RunCode,
}

pub struct Playground {
    link: ComponentLink<Self>,
}
impl Component for Playground {
    type Message = PlaygroundMessage;
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
