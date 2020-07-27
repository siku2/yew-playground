use yew::{html, Component, ComponentLink, Properties, ShouldRender};

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

    fn view(&self) -> yew::Html {
        html! {
            <textarea />
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FileProps {}

pub struct File {}
impl Component for File {
    type Message = ();
    type Properties = FileProps;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        html! {}
    }
}
