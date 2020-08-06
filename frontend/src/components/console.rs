use crate::utils::NeqAssign;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ConsoleProps {}

#[derive(Debug)]
pub struct Console {
    props: ConsoleProps,
}
impl Component for Console {
    type Message = ();
    type Properties = ConsoleProps;

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
        html! {}
    }
}
