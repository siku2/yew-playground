use std::{
    borrow::Borrow,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    iter,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorName {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
impl ColorName {
    fn from_code(code: usize) -> Option<Self> {
        use ColorName::*;
        [Black, Red, Green, Yellow, Blue, Magenta, Cyan, White]
            .get(code % 10)
            .copied()
    }
}
impl Display for ColorName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use ColorName::*;
        let name = match self {
            Black => "black",
            Red => "red",
            Green => "green",
            Yellow => "yellow",
            Blue => "blue",
            Magenta => "magenta",
            Cyan => "cyan",
            White => "white",
        };
        f.write_str(name)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClassStyle {
    pub class: String,
    pub style: Option<String>,
}
impl ClassStyle {
    fn push_class(&mut self, new_class: &str) {
        let class = &mut self.class;
        if !class.is_empty() {
            class.push(' ');
        }
        class.push_str(new_class);
    }

    fn push_style(&mut self, new_style: &str) {
        if cfg!(debug_assertions) {
            if let Some(style) = &self.style {
                assert!(style.ends_with(';'), "existing style doesn't end in ';'");
            }
        }

        if let Some(style) = &mut self.style {
            style.push_str(new_style);
        } else {
            self.style = Some(new_style.to_owned());
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sgr {
    Reset,
    Bold,
    BoldOff,
    Italic,
    ItalicOff,
    Underline,
    UnderlineOff,
    ColorFgRgb(u32),
    ColorFgName(ColorName),
    ColorFgNameBright(ColorName),
    ResetColorFg,
    ColorBgRgb(u32),
    ColorBgName(ColorName),
    ColorBgNameBright(ColorName),
    ResetColorBg,
}
impl Sgr {
    fn color_rgb(mut params: impl Iterator<Item = usize>, background: bool) -> Option<Self> {
        if params.next() != Some(2) {
            return None;
        }

        let (r, g, b) = (params.next()?, params.next()?, params.next()?);
        let rgb = u32::try_from((r << 16) + (g << 8) + b).ok()?;
        if background {
            Some(Self::ColorBgRgb(rgb))
        } else {
            Some(Self::ColorFgRgb(rgb))
        }
    }

    pub fn from_params(mut params: impl Iterator<Item = usize>) -> Option<Self> {
        use Sgr::*;
        let code = params.next()?;
        Some(match code {
            0 => Reset,
            1 => Bold,
            3 => Italic,
            4 => Underline,
            22 => BoldOff,
            23 => ItalicOff,
            24 => UnderlineOff,
            30..=37 => ColorFgName(ColorName::from_code(code)?),
            38 => Self::color_rgb(params, false)?,
            39 => ResetColorFg,
            40..=47 => ColorBgName(ColorName::from_code(code)?),
            48 => Self::color_rgb(params, true)?,
            49 => ResetColorBg,
            90..=97 => ColorFgNameBright(ColorName::from_code(code)?),
            100..=107 => ColorBgNameBright(ColorName::from_code(code)?),
            _ => return None,
        })
    }
}

pub fn parse_csi_params(mut params: impl Iterator<Item = usize>) -> Vec<Sgr> {
    iter::from_fn(|| Sgr::from_params(&mut params)).collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ColorEffect {
    None,
    Name(ColorName),
    NameBright(ColorName),
    Rgb(u32),
}
impl ColorEffect {
    fn add_to_style(&self, style: &mut ClassStyle, foreground: bool) {
        let ground_mod = if foreground { "fg" } else { "bg" };

        match self {
            Self::None => {}
            Self::Name(name) => style.push_class(&format!("ansi--{}-{}", name, ground_mod)),
            Self::NameBright(name) => {
                style.push_class(&format!("ansi--bright-{}-{}", name, ground_mod))
            }
            Self::Rgb(rgb) => {
                style.push_class(&format!("ansi--rgb-{}", ground_mod));
                style.push_style(&format!("--value: #{:06x};", rgb));
            }
        }
    }
}
impl Default for ColorEffect {
    fn default() -> Self {
        Self::None
    }
}

impl From<&Sgr> for ColorEffect {
    fn from(sgr: &Sgr) -> Self {
        use Sgr::*;
        match sgr {
            ColorFgRgb(rgb) | ColorBgRgb(rgb) => Self::Rgb(*rgb),
            ColorFgName(name) | ColorBgName(name) => Self::Name(*name),
            ColorFgNameBright(name) | ColorBgNameBright(name) => Self::NameBright(*name),
            _ => Self::None,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SgrEffect {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub fg: ColorEffect,
    pub bg: ColorEffect,
}
impl SgrEffect {
    fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn apply_sgr(&mut self, sgr: impl Borrow<Sgr>) {
        use Sgr::*;
        let sgr = sgr.borrow();
        match sgr {
            Reset => self.reset(),
            Bold => self.bold = true,
            BoldOff => self.bold = false,
            Italic => self.italic = true,
            ItalicOff => self.italic = false,
            Underline => self.underline = true,
            UnderlineOff => self.underline = false,
            ColorFgRgb(_) | ColorFgName(_) | ColorFgNameBright(_) | ResetColorFg => {
                self.fg = ColorEffect::from(sgr);
            }
            ColorBgRgb(_) | ColorBgName(_) | ColorBgNameBright(_) | ResetColorBg => {
                self.bg = ColorEffect::from(sgr);
            }
        }
    }

    pub fn apply_sgrs<T: Borrow<Sgr>>(&mut self, sgrs: impl IntoIterator<Item = T>) {
        for sgr in sgrs {
            self.apply_sgr(sgr.borrow());
        }
    }

    pub fn to_style(&self) -> ClassStyle {
        let mut style = ClassStyle::default();

        if self.bold {
            style.push_class("ansi--bold");
        }
        if self.italic {
            style.push_class("ansi--italic");
        }
        if self.underline {
            style.push_class("ansi--underline");
        }

        self.fg.add_to_style(&mut style, true);
        self.bg.add_to_style(&mut style, false);

        style
    }
}
