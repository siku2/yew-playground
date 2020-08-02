use crate::{
    routes::AppRouter,
    services::locale::{self, LoadBundleTask},
};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[derive(Debug)]
pub struct Root;
impl Component for Root {
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
            <AppRouter />
        }
    }
}

#[derive(Debug)]
pub enum BootMessage {
    BundleLoaded(anyhow::Result<()>),
}

#[derive(Debug)]
pub struct BootComponent {
    load_bundle_task: Option<LoadBundleTask>,
}
impl Component for BootComponent {
    type Message = BootMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let load_bundle_task =
            locale::load_default_bundle(link.callback(BootMessage::BundleLoaded)).map_or_else(
                |err| {
                    log::error!("failed to start loading fluent bundle: {}", err);
                    None
                },
                Some,
            );
        Self { load_bundle_task }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BootMessage::BundleLoaded(res) => {
                if let Err(err) = res {
                    log::error!("error while loading bundle: {}", err);
                }
                self.load_bundle_task = None;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.load_bundle_task.is_some() {
            html! {}
        } else {
            html! { <Root /> }
        }
    }
}
