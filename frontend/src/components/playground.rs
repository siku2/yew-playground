use super::editor::Editor;
use crate::services::{
    api::{self, Channel, CompileRequest, CompileResponse, Mode},
    locale,
};
use yew::{html, services::fetch::FetchTask, Component, ComponentLink, Html, ShouldRender};

#[derive(Debug)]
pub enum PlaygroundMessage {
    RunCode,
    OnCompileResponse(anyhow::Result<CompileResponse>),
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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use PlaygroundMessage::*;

        match msg {
            RunCode => {
                // let task = api::compile_with_request(
                //     &CompileRequest {
                //         channel: Channel::Stable,
                //         mode: Mode::Debug,
                //         edition: None,
                //         backtrace: false,
                //     },
                //     self.link.callback(OnCompileResponse),
                // );
                // self.fetch_task = task.ok();
                false
            }
            OnCompileResponse(res) => {
                self.fetch_task = None;
                log::info!("{:?}", res);
                true
            }
        }
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
