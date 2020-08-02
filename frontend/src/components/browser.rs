use crate::{services::api::SessionRef, utils::NeqAssign};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

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
    url: String,
}
impl Component for Browser {
    type Message = BrowserMsg;
    type Properties = BrowserProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // TODO get proper url
        let url = format!("http://localhost:8000/proxy/{}/", props.session.id);
        Self { props, url }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <div class="browser">
                <iframe src=&self.url />
            </div>
        }
    }
}
