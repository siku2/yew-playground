use crate::{
    components::{explorer::Explorer, playground::Playground},
    services::api::{Session, SessionRef},
    utils::NeqAssign,
};
use std::rc::Rc;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SandboxPageProps {
    pub id: String,
}

pub struct SandboxPage {
    props: SandboxPageProps,
    session: SessionRef,
}
impl Component for SandboxPage {
    type Message = ();
    type Properties = SandboxPageProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let session = Rc::new(Session::new(props.id.clone()));
        Self { props, session }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Explorer session=Rc::clone(&self.session) />
                <Playground />
            </>
        }
    }
}
