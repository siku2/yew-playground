use home::HomePage;
use sandbox::SandboxPage;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{router::Router, Switch};

mod home;
mod sandbox;

#[derive(Clone, Debug, Switch)]
pub enum AppSwitch {
    #[to = "/s/{id}"]
    Sandbox(String),
    #[to = "/"]
    Home,
}

#[derive(Clone, Debug)]
pub struct AppRouter;
impl AppRouter {
    fn render_route(switch: AppSwitch) -> Html {
        use AppSwitch::*;

        match switch {
            Home => html! { <HomePage /> },
            Sandbox(id) => html! { <SandboxPage id=id /> },
        }
    }
}
impl Component for AppRouter {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppSwitch> render=Router::render(Self::render_route) />
        }
    }
}
