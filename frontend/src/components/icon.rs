use crate::utils::NeqAssign;
use std::fmt::{self, Display, Formatter};
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct MdiProps {
    pub icon: Icon,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or(true)]
    pub active: bool,
}

#[derive(Debug)]
pub struct Mdi {
    props: MdiProps,
    classes: Classes,
}
impl Mdi {
    fn update_classes(&mut self) {
        let MdiProps {
            icon: _,
            size,
            color,
            active,
        } = &self.props;

        let mut classes = Classes::from("material-icons");

        if let Some(size) = size {
            classes.push(size.to_value());
        }

        if let Some(color) = color {
            classes.push(color.to_value());
        }

        if !active {
            classes.push("md-inactive");
        }

        self.classes = classes;
    }
}
impl Component for Mdi {
    type Message = ();
    type Properties = MdiProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut this = Self {
            props,
            classes: Classes::default(),
        };
        this.update_classes();
        this
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.neq_assign(props) {
            self.update_classes();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let icon = self.props.icon;
        html! {
            <i class=self.classes.clone()>{ icon }</i>
        }
    }
}

macro_rules! str_enum {
    (
        $(#[$meta:meta])*
        pub enum $ident:ident {
            $(
                $(#[$v_meta:meta])*
                $v_ident:ident = $v_value:literal,
            )+
        }
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum $ident {
            $(
                $(#[$v_meta])*
                $v_ident,
            )*
        }
        impl $ident {
            /// Get the variant value
            pub fn to_value(&self) -> &'static str {
                match self {
                    $(
                        Self::$v_ident => $v_value,
                    )*
                }
            }
        }
        impl Display for $ident {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str(self.to_value())
            }
        }
    };
}

str_enum! {
    pub enum Size {
        Md18 = "md-18",
        Md24 = "md-24",
        Md36 = "md-36",
        Md48 = "md-48",
    }
}

str_enum! {
    pub enum Color {
        Dark = "md-dark",
        Light = "md-light",
    }
}

str_enum! {
    pub enum Icon {
        ChevronDown = "expand_more",
        ChevronRight = "chevron_right",
        CircleMedium = "circle_medium",
        Close = "close",
    }
}
