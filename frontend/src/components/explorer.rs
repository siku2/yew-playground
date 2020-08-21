use super::icon::{Icon, Mdi};
use crate::{
    services::{
        api::{Session, SessionRef},
        locale,
    },
    utils::NeqAssign,
};
use protocol::SandboxStructure;
use std::rc::Rc;
use yew::{
    html,
    services::fetch::FetchTask,
    Callback,
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
    pub onclick_file: Callback<Rc<protocol::File>>,
}

#[derive(Debug)]
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
                let onclick_file = &self.props.onclick_file;
                html! {
                    <nav class="explorer">
                        <span class="explorer__header">{ locale::get("explorer-header", None) }</span>
                        <Directory onclick_file=onclick_file.clone() start_open=true directory=Rc::clone(public) />
                        <Directory onclick_file=onclick_file.clone() start_open=true directory=Rc::clone(src) />
                    </nav>
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

#[derive(Debug)]
pub enum DirectoryMsg {
    ToggleOpen,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct DirectoryProps {
    pub directory: Rc<protocol::Directory>,
    pub onclick_file: Callback<Rc<protocol::File>>,
    #[prop_or_default]
    pub start_open: bool,
}

#[derive(Debug)]
pub struct Directory {
    props: DirectoryProps,
    link: ComponentLink<Self>,
    directories: Vec<Rc<protocol::Directory>>,
    files: Vec<Rc<protocol::File>>,
    open: bool,
}
impl Directory {
    fn rebuild_cache(&mut self) {
        let protocol::Directory {
            directories, files, ..
        } = &*self.props.directory;
        self.directories = directories.iter().cloned().map(Rc::new).collect();
        self.files = files.iter().cloned().map(Rc::new).collect();
    }

    fn view_content(&self) -> Html {
        let onclick_file = &self.props.onclick_file;

        let dir_comps = self.directories.iter().map(|dir| {
            html! { <Directory key=dir.path.clone() onclick_file=onclick_file.clone() directory=Rc::clone(dir) /> }
        });
        let file_comps = self.files.iter().map(|file| {
            html! { <File key=file.path.clone() onclick=onclick_file.clone() file=Rc::clone(file) /> }
        });

        html! {
            <div class="explorer-dir__content">
                { for dir_comps }
                { for file_comps }
            </div>
        }
    }
}
impl Component for Directory {
    type Message = DirectoryMsg;
    type Properties = DirectoryProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let open = props.start_open;
        let mut instance = Self {
            props,
            link,
            directories: Vec::new(),
            files: Vec::new(),
            open,
        };

        instance.rebuild_cache();
        instance
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use DirectoryMsg::*;
        match msg {
            ToggleOpen => {
                self.open = !self.open;
                true
            }
        }
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
        let directory = &self.props.directory;

        let (content, icon);
        if self.open {
            content = self.view_content();
            icon = Icon::ChevronDown;
        } else {
            content = html! {};
            icon = Icon::ChevronRight;
        };

        let onclick_name = self.link.callback(|_| DirectoryMsg::ToggleOpen);

        html! {
            <div class="explorer-dir">
                <span class="explorer-item__name" onclick=onclick_name>
                    <Mdi icon=icon />
                    { &directory.name }
                </span>
                { content }
            </div>
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FileProps {
    pub file: Rc<protocol::File>,
    pub onclick: Callback<Rc<protocol::File>>,
}

#[derive(Debug)]
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
        let FileProps { file, onclick } = &self.props;

        let onclick = {
            let file = Rc::clone(file);
            let onclick = onclick.clone();
            Callback::from(move |_| {
                onclick.emit(Rc::clone(&file));
            })
        };

        html! {
            <div class="explorer-file" onclick=onclick>
                <span class="explorer-item__name">{ &file.name }</span>
            </div>
        }
    }
}
