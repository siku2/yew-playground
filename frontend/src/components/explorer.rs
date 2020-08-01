use crate::{
    services::api::{Session, SessionRef},
    utils::NeqAssign,
};
use protocol::SandboxStructure;
use std::rc::Rc;
use yew::{
    html,
    services::fetch::FetchTask,
    Component,
    ComponentLink,
    Html,
    Properties,
    ShouldRender,
};

#[derive(Debug)]
pub enum ExplorerMsg {
    StructureLoaded(anyhow::Result<SandboxStructure>),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ExplorerProps {
    pub session: SessionRef,
}

pub struct Explorer {
    props: ExplorerProps,
    link: ComponentLink<Self>,
    state: ExplorerState,
}
impl Component for Explorer {
    type Message = ExplorerMsg;
    type Properties = ExplorerProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = ExplorerState::start(&props.session, link.clone());
        Self { props, link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state.update(msg)
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.session.id != props.session.id {
            // reset state if the session id changes
            self.state = ExplorerState::start(&self.props.session, self.link.clone());
        }

        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        use ExplorerState::*;
        match &self.state {
            Loading(_) => {
                // TODO render loading state
                html! { "WIP: LOADING" }
            }
            Failed(_) => {
                // TODO render error state
                html! { "WIP: FAILED" }
            }
            Loaded { public, src } => {
                html! {
                    <div class="explorer">
                        <Directory directory=Rc::clone(public) />
                        <Directory directory=Rc::clone(src) />
                    </div>
                }
            }
        }
    }
}

#[derive(Debug)]
enum ExplorerState {
    Loading(FetchTask),
    Failed(anyhow::Error),
    Loaded {
        public: Rc<protocol::Directory>,
        src: Rc<protocol::Directory>,
    },
}
impl ExplorerState {
    pub fn start(session: &Session, link: ComponentLink<Explorer>) -> Self {
        Self::Loading(
            session
                .get_structure(link.callback(ExplorerMsg::StructureLoaded))
                .expect("failed to request structure"),
        )
    }

    pub fn update(&mut self, msg: ExplorerMsg) -> ShouldRender {
        use ExplorerMsg::*;

        match msg {
            StructureLoaded(Ok(resp)) => {
                let public = Rc::new(resp.public);
                let src = Rc::new(resp.src);
                *self = Self::Loaded { public, src };
                true
            }
            StructureLoaded(Err(err)) => {
                log::error!("loading sandbox structure failed: {}", err);
                *self = Self::Failed(err);
                true
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct DirectoryProps {
    directory: Rc<protocol::Directory>,
}

pub struct Directory {
    props: DirectoryProps,
    directories: Vec<Rc<protocol::Directory>>,
    files: Vec<Rc<protocol::File>>,
}
impl Directory {
    fn rebuild_cache(&mut self) {
        let dir = &self.props.directory;
        self.directories = dir.directories.iter().cloned().map(Rc::new).collect();
        self.files = dir.files.iter().cloned().map(Rc::new).collect();
    }
}
impl Component for Directory {
    type Message = ();
    type Properties = DirectoryProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut instance = Self {
            props,
            directories: Vec::new(),
            files: Vec::new(),
        };

        instance.rebuild_cache();
        instance
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.neq_assign(props) {
            self.rebuild_cache();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let dir_comps = self.directories.iter().map(|dir| {
            html! { <Directory directory=Rc::clone(dir) /> }
        });
        let file_comps = self.files.iter().map(|file| {
            html! { <File file=Rc::clone(file) /> }
        });

        let directory = &self.props.directory;

        html! {
            <div class="explorer-dir">
                <span class="explorer-item__name">{ &directory.name }</span>
                <div class="explorer-dir__content">
                    { for dir_comps }
                    { for file_comps }
                </div>
            </div>
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
            <div class="explorer-file">
                <span class="explorer-item__name">{ &file.name }</span>
            </div>
        }
    }
}
