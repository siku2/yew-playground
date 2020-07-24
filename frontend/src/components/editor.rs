use yew::{html, Component, ComponentLink, Properties, ShouldRender};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EditorProps {}

pub struct Editor {}
impl Component for Editor {
    type Message = ();
    type Properties = EditorProps;

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
