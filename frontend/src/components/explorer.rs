use crate::utils::NeqAssign;
use std::rc::Rc;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ExplorerProps {}

pub struct Explorer {}
impl Component for Explorer {
    type Message = ();
    type Properties = ExplorerProps;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <textarea />
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FileProps {
    file: Rc<protocol::File>,
}

pub struct File {
    props: FileProps,
}
impl Component for File {
    type Message = ();
    type Properties = FileProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let file = &self.props.file;

        html! {
            <div class="explorer__file">
                { &file.name }
            </div>
        }
    }
}
