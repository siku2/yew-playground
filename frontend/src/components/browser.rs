use crate::{
    services::{api::SessionRef, locale},
    utils::{ComponentRef, NeqAssign},
};
use web_sys::{HtmlIFrameElement, Window};
use yew::{html, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Controller(ComponentRef<Browser>);
impl Controller {
    fn populate(&self, link: ComponentLink<Browser>) {
        self.0.populate(link);
    }

    pub fn reload(&self) -> bool {
        self.0.send_message(BrowserMsg::Reload)
    }
}

#[derive(Debug)]
pub enum BrowserMsg {
    Reload,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BrowserProps {
    pub session: SessionRef,
    pub controller: Controller,
}

#[derive(Debug)]
pub struct Browser {
    props: BrowserProps,
    link: ComponentLink<Self>,
    url: String,
    iframe_ref: NodeRef,
}
impl Browser {
    fn iframe(&self) -> HtmlIFrameElement {
        self.iframe_ref.cast().expect("failed to get iframe")
    }

    fn iframe_window(&self) -> Window {
        self.iframe()
            .content_window()
            .expect("unable to get content window")
    }
}
impl Component for Browser {
    type Message = BrowserMsg;
    type Properties = BrowserProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.controller.populate(link.clone());

        // TODO get proper url
        let url = format!("http://localhost:8000/proxy/{}/", props.session.id);
        Self {
            props,
            link,
            url,
            iframe_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use BrowserMsg::*;
        match msg {
            Reload => {
                self.iframe_window()
                    .location()
                    .reload()
                    .expect("unable to reload");

                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let Self {
            link,
            url,
            iframe_ref,
            ..
        } = self;
        let onclick_reload = link.callback(|_| BrowserMsg::Reload);

        html! {
            <div class="browser">
                <div class="browser__toolbar">
                    <button onclick=onclick_reload>
                        { locale::get("browser-reload", None) }
                    </button>
                    <span>{ url }</span>
                </div>
                <iframe ref=iframe_ref.clone() class="browser__frame" title=locale::get("browser-iframe-title", None) src=&self.url />
            </div>
        }
    }
}
