use crate::{
    services::{api::SessionRef, locale},
    utils::NeqAssign,
};
use web_sys::HtmlIFrameElement;
use yew::{html, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender};

#[derive(Debug)]
pub enum BrowserMsg {
    Reload,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BrowserProps {
    pub session: SessionRef,
}

#[derive(Debug)]
pub struct Browser {
    props: BrowserProps,
    link: ComponentLink<Self>,
    url: String,
    iframe_ref: NodeRef,
}
impl Component for Browser {
    type Message = BrowserMsg;
    type Properties = BrowserProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
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
                let iframe = self
                    .iframe_ref
                    .cast::<HtmlIFrameElement>()
                    .expect("failed to get iframe");
                iframe
                    .content_window()
                    .expect("unable to get content window")
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
