use super::{action_bar::ActionBar, explorer::Explorer};
use crate::{
    services::api::{Session, SessionRef},
    utils::NeqAssign,
};
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

type TabIdentifier = usize;

#[derive(Debug)]
pub enum EditorMsg {
    OpenFile(Rc<protocol::File>),
    FileResponse(TabIdentifier, anyhow::Result<String>),
    SelectTab(TabIdentifier),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EditorProps {
    pub session: SessionRef,
}

#[derive(Debug)]
pub struct Editor {
    props: EditorProps,
    link: ComponentLink<Self>,
    next_tab_id: TabIdentifier,
    tabs: Vec<EditorTab>,
    selected: Option<TabIdentifier>,
}
impl Editor {
    fn generate_next_tab_id(&mut self) -> TabIdentifier {
        let id = self.next_tab_id;
        self.next_tab_id = id.wrapping_add(1);
        id
    }

    fn get_tab(&self, id: TabIdentifier) -> Option<&EditorTab> {
        self.tabs.iter().find(|tab| tab.id == id)
    }

    fn get_tab_mut(&mut self, id: TabIdentifier) -> Option<&mut EditorTab> {
        self.tabs.iter_mut().find(|tab| tab.id == id)
    }

    fn force_create_tab(&mut self, file: Rc<protocol::File>) -> TabIdentifier {
        let id = self.generate_next_tab_id();
        let content = ContentState::start(
            &self.props.session,
            &file.path,
            self.link
                .callback(move |resp| EditorMsg::FileResponse(id, resp)),
        );
        let tab = EditorTab {
            id,
            file,
            content,
            dirty: false,
        };
        self.tabs.push(tab);
        id
    }

    fn find_or_create_tab(&mut self, file: Rc<protocol::File>) -> TabIdentifier {
        self.tabs
            .iter()
            .find_map(|tab| if tab.file == file { Some(tab.id) } else { None })
            .unwrap_or_else(|| self.force_create_tab(file))
    }

    fn render_tab(&self, tab: &EditorTab) -> Html {
        let mut classes = vec!["htbar__tab"];
        if matches!(self.selected, Some(id) if id == tab.id) {
            classes.push("htbar__tab--selected");
        }
        if tab.dirty {
            classes.push("htbar__tab--dirty");
        }

        let onclick = {
            let id = tab.id;
            self.link.callback(move |_| EditorMsg::SelectTab(id))
        };

        html! {
            <div key=tab.file.path.clone() class=classes role="tab" onclick=onclick>
                { &tab.file.name }
            </div>
        }
    }

    fn view_editor_window(&self) -> Html {
        let tab_comps = self.tabs.iter().map(|tab| self.render_tab(tab));

        html! {
            <div class="editor-window">
                <nav class="htbar htbar--scroll" role="tablist">
                    { for tab_comps }
                </nav>
                <div class="editor-window__content">
                    { self.view_content() }
                </div>
            </div>
        }
    }

    fn view_content(&self) -> Html {
        if let Some(selected) = self.selected {
            self.view_tab_content(self.get_tab(selected).expect("selected tab doesn't exist"))
        } else {
            // TODO render welcome
            html! {
                "WIP: no tab selected"
            }
        }
    }

    fn view_tab_content(&self, tab: &EditorTab) -> Html {
        use ContentState::*;
        match &tab.content {
            Loading(_) => {
                // TODO render loading state
                html! { "WIP: loading content" }
            }
            Failed(err) => {
                // TODO render error state
                html! {
                    { format!("WIP: failed: {}", err) }
                }
            }
            Loaded(content) => {
                html! {
                    <textarea>
                        { content }
                    </textarea>
                }
            }
        }
    }
}
impl Component for Editor {
    type Message = EditorMsg;
    type Properties = EditorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            next_tab_id: 0,
            tabs: Vec::new(),
            selected: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use EditorMsg::*;
        match msg {
            OpenFile(file) => {
                let id = self.find_or_create_tab(file);
                self.selected = Some(id);
                true
            }
            FileResponse(id, resp) => {
                if let Some(tab) = self.get_tab_mut(id) {
                    tab.content.handle_response(resp);
                    true
                } else {
                    log::debug!("received response for tab which no longer exists: {}", id);
                    false
                }
            }
            SelectTab(id) => {
                self.selected = Some(id);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let EditorProps { session } = &self.props;
        let onclick_file = self.link.callback(EditorMsg::OpenFile);
        html! {
            <div class="editor">
                <Explorer session=Rc::clone(session) onclick_file=onclick_file />
                { self.view_editor_window() }
                <ActionBar session=Rc::clone(session) oncompile=Callback::noop() />
            </div>
        }
    }
}

#[derive(Debug)]
struct EditorTab {
    id: TabIdentifier,
    file: Rc<protocol::File>,
    content: ContentState,
    dirty: bool,
}

#[derive(Debug)]
enum ContentState {
    Loading(FetchTask),
    Failed(anyhow::Error),
    Loaded(String),
}
impl ContentState {
    fn start(session: &Session, path: &str, callback: Callback<anyhow::Result<String>>) -> Self {
        Self::Loading(
            session
                .get_file(path, callback)
                .expect("failed to create request for file"),
        )
    }

    fn handle_response(&mut self, resp: anyhow::Result<String>) {
        match resp {
            Ok(content) => {
                *self = Self::Loaded(content);
            }
            Err(err) => {
                log::error!("error loading file: {}", err);
                *self = Self::Failed(err);
            }
        }
    }
}
