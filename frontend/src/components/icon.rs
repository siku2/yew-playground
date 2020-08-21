use crate::utils::NeqAssign;
use std::fmt::{self, Display, Formatter};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct MdiProps {
    pub icon: Icon,
}

#[derive(Debug)]
pub struct Mdi {
    props: MdiProps,
}
impl Component for Mdi {
    type Message = ();
    type Properties = MdiProps;

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
        let icon = self.props.icon;
        html! {
            <i class="material-icons">{ icon }</i>
        }
    }
}

macro_rules! icon_enum {
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
            /// Get the font icon name.
            pub fn icon_name(&self) -> &'static str {
                match self {
                    $(
                        Self::$v_ident => $v_value,
                    )*
                }
            }
        }
        impl Display for $ident {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str(self.icon_name())
            }
        }
    };
}

icon_enum! {
    pub enum Icon {
        /// Chevron down.
        ChevronDown = "expand_more",
        /// Chevron right.
        ChevronRight = "chevron_right",
        /// Circle Medium.
        CircleMedium = "circle_medium",
        /// Close.
        Close = "close",
    }
}
