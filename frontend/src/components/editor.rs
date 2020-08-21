use super::{action_bar::ActionBar, explorer::Explorer};
use crate::{
    services::{
        api::{Session, SessionRef},
        locale,
    },
    utils::NeqAssign,
};
use monaco::{
    api::{CodeEditorOptions, TextModel},
    sys::Uri,
    yew::CodeEditor,
};
use protocol::CompileResponse;
use std::{rc::Rc, slice};
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
    SaveTab(TabIdentifier),
    SaveResponse(TabIdentifier, anyhow::Result<()>),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EditorProps {
    pub session: SessionRef,
    pub oncompile: Callback<CompileResponse>,
}

#[derive(Debug)]
pub struct Editor {
    props: EditorProps,
    link: ComponentLink<Self>,
    tabs: Tabs,
    selected: Option<TabIdentifier>,
    monaco_options: Rc<CodeEditorOptions>,
}
impl Editor {
    fn render_tab(&self, tab: &Tab) -> Html {
        let mut classes = vec!["htbar__tab"];
        if matches!(self.selected, Some(id) if id == tab.id) {
            classes.push("htbar__tab--selected");
        }
        if tab.dirty {
            classes.push("htbar__tab--dirty");
        }

        let tab_id = tab.id;
        let onclick_tab = self.link.callback(move |_| EditorMsg::SelectTab(tab_id));
        let onclick_save = self.link.callback(move |_| EditorMsg::SaveTab(tab_id));

        html! {
            <div key=tab.file.path.clone() class=classes role="tab" onclick=onclick_tab>
                { &tab.file.name }
                <button onclick=onclick_save>
                    { locale::get("editor-save", None) }
                </button>
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
            self.view_tab_content(self.tabs.get(selected).expect("selected tab doesn't exist"))
        } else {
            // TODO render welcome
            html! {
                "WIP: no tab selected"
            }
        }
    }

    fn view_tab_content(&self, tab: &Tab) -> Html {
        use ContentState::*;
        // TODO dirty flag needs to be set again
        match &tab.state {
            Loading(_) => {
                // TODO render loading state
                html! { "WIP: loading" }
            }
            Failed(err) => {
                // TODO render error state
                html! {
                    { format!("WIP: failed: {}", err) }
                }
            }
            Idle => {
                let model = tab.model.clone();
                // let tab_id = tab.id;
                // let oninput = self.link.callback(move |input: InputData| {
                //     EditorMsg::ChangeTabContent(tab_id, input.value)
                // });
                // TODO this is very much WIP
                html! {
                    <CodeEditor options=Rc::clone(&self.monaco_options) model=model />
                }
            }
        }
    }
}
impl Component for Editor {
    type Message = EditorMsg;
    type Properties = EditorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let monaco_options = Rc::new(CodeEditorOptions::default().with_theme("vs-dark".to_owned()));
        Self {
            props,
            link,
            tabs: Tabs::new(),
            selected: None,
            monaco_options,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use EditorMsg::*;
        match msg {
            OpenFile(file) => {
                let id = self
                    .tabs
                    .find_or_create(&self.props.session, &self.link, file);
                self.selected = Some(id);
                true
            }
            FileResponse(id, resp) => {
                if let Some(tab) = self.tabs.get_mut(id) {
                    tab.handle_load_response(resp);
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
            SaveTab(id) => {
                if let Some(tab) = self.tabs.get_mut(id) {
                    let ok = tab.save(
                        &self.props.session,
                        self.link
                            .callback(move |resp| EditorMsg::SaveResponse(id, resp)),
                    );
                    if !ok {
                        log::warn!("can't save file");
                    }
                    true
                } else {
                    false
                }
            }
            SaveResponse(id, resp) => {
                if let Some(tab) = self.tabs.get_mut(id) {
                    tab.handle_save_response(resp);
                    true
                } else {
                    log::debug!("received response for tab which no longer exists: {}", id);
                    false
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let EditorProps {
            session, oncompile, ..
        } = &self.props;
        let onclick_file = self.link.callback(EditorMsg::OpenFile);
        html! {
            <div class="editor">
                <Explorer session=Rc::clone(session) onclick_file=onclick_file />
                { self.view_editor_window() }
                <ActionBar session=Rc::clone(session) oncompile=oncompile.clone() />
            </div>
        }
    }
}

#[derive(Debug)]
struct Tabs {
    tabs: Vec<Tab>,
    next_tab_id: usize,
}
impl Tabs {
    fn new() -> Self {
        Self {
            tabs: Vec::new(),
            next_tab_id: 0,
        }
    }

    fn generate_tab_id(&mut self) -> TabIdentifier {
        let id = self.next_tab_id;
        self.next_tab_id = id.wrapping_add(1);
        id
    }

    fn iter(&self) -> slice::Iter<'_, Tab> {
        self.tabs.iter()
    }

    fn get(&self, id: TabIdentifier) -> Option<&Tab> {
        self.tabs.iter().find(|tab| tab.id == id)
    }

    fn get_mut(&mut self, id: TabIdentifier) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|tab| tab.id == id)
    }

    fn create(
        &mut self,
        session: &Session,
        link: &ComponentLink<Editor>,
        file: Rc<protocol::File>,
    ) -> TabIdentifier {
        let id = self.generate_tab_id();
        let tab = Tab::open(
            session,
            id,
            file,
            link.callback(move |resp| EditorMsg::FileResponse(id, resp)),
        );
        self.tabs.push(tab);
        id
    }

    fn find_or_create(
        &mut self,
        session: &Session,
        link: &ComponentLink<Editor>,
        file: Rc<protocol::File>,
    ) -> TabIdentifier {
        self.iter()
            .find_map(|tab| if tab.file == file { Some(tab.id) } else { None })
            .unwrap_or_else(|| self.create(session, link, file))
    }
}

#[derive(Debug)]
struct Tab {
    id: TabIdentifier,
    file: Rc<protocol::File>,
    model: Option<TextModel>,
    state: ContentState,
    dirty: bool,
}
impl Tab {
    fn open(
        session: &Session,
        id: TabIdentifier,
        file: Rc<protocol::File>,
        callback: Callback<anyhow::Result<String>>,
    ) -> Self {
        let state = ContentState::load(session, &file.path, callback);
        Self {
            id,
            file,
            model: None,
            state,
            dirty: false,
        }
    }

    fn save(&mut self, session: &Session, callback: Callback<anyhow::Result<()>>) -> bool {
        if self.state.is_loading() {
            return false;
        }

        if let Some(model) = &self.model {
            self.state = ContentState::save(session, &self.file.path, model.get_value(), callback);
            true
        } else {
            false
        }
    }

    fn handle_load_response(&mut self, resp: anyhow::Result<String>) {
        let state = &mut self.state;
        if !state.is_loading() {
            log::debug!("ignoring response: not loading");
            return;
        }

        match resp {
            Ok(content) => {
                let uri = Uri::file(&self.file.path);
                let model = TextModel::create(&content, None, Some(&uri));
                self.model = Some(model);
                *state = ContentState::Idle;
            }
            Err(err) => {
                log::error!("error loading file: {}", err);
                *state = ContentState::Failed(err);
            }
        }
    }

    fn handle_save_response(&mut self, resp: anyhow::Result<()>) {
        let state = &mut self.state;
        if !state.is_loading() {
            log::debug!("ignoring response: not loading");
            return;
        }

        match resp {
            Ok(_) => {
                *state = ContentState::Idle;
                self.dirty = false;
            }
            Err(err) => {
                log::error!("error saving file: {}", err);
                *state = ContentState::Failed(err);
            }
        }
    }
}

#[derive(Debug)]
enum ContentState {
    Loading(FetchTask),
    Failed(anyhow::Error),
    Idle,
}
impl ContentState {
    fn load(session: &Session, path: &str, callback: Callback<anyhow::Result<String>>) -> Self {
        Self::Loading(
            session
                .get_file(path, callback)
                .expect("failed to create request for file"),
        )
    }

    fn save(
        session: &Session,
        path: &str,
        content: String,
        callback: Callback<anyhow::Result<()>>,
    ) -> Self {
        Self::Loading(
            session
                .put_file(path, content, callback)
                .expect("failed to create save request"),
        )
    }

    fn is_loading(&self) -> bool {
        matches!(self, Self::Loading(_))
    }
}
