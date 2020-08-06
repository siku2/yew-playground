use crate::utils::NeqAssign;
use graphic_rendition::{ClassStyle, SgrEffect};
use sequences::{Csi, Escape, Marker};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

mod cursor;
mod graphic_rendition;
mod sequences;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AnsiProps {
    pub text: String,
}

#[derive(Debug)]
pub struct Ansi {
    props: AnsiProps,
    parts: Vec<Part>,
}
impl Ansi {
    fn view_part(part: &Part) -> Html {
        let Part {
            content,
            style: ClassStyle { class, style },
        } = part;

        // TODO update to optional attribute when they land
        let style = style.clone().unwrap_or_default();
        html! {
            <span class=class.clone() style=style>
                { content }
            </span>
        }
    }

    fn update_parts(&mut self) {
        let s = &self.props.text;
        self.parts.clear();
        let mut effect = SgrEffect::default();
        for marker in sequences::get_markers(s) {
            match marker {
                Marker::Text(content) => {
                    self.parts
                        .push(Part::new_from_effect(content.to_owned(), &effect));
                }
                Marker::Sequence(Escape::Csi(Csi::Sgr(sgrs))) => {
                    effect.apply_sgrs(sgrs);
                }
            }
        }
    }
}
impl Component for Ansi {
    type Message = ();
    type Properties = AnsiProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut instance = Self {
            props,
            parts: Vec::new(),
        };
        instance.update_parts();
        instance
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.neq_assign(props) {
            self.update_parts();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let part_comps = self.parts.iter().map(Self::view_part);
        html! {
            <div class="ansi-container">
                { for part_comps }
            </div>
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct Part {
    content: String,
    style: ClassStyle,
}
impl Part {
    fn new_from_effect(content: String, effect: &SgrEffect) -> Self {
        Self {
            content,
            style: effect.to_style(),
        }
    }
}
