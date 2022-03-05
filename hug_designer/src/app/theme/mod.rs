pub mod colors;

pub mod light;
mod dark;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

macro_rules! impl_from {
    ($name:ident, $stylename:ident) => {
        impl From<Theme> for Box<dyn iced::$name::StyleSheet> {
            fn from(theme: Theme) -> Self {
                match theme {
                    Theme::Dark => dark::$stylename.into(),
                }
            }
        }
    }
}

macro_rules! impl_from_lt {
    ($name:ident, $stylename:ident) => {
        impl<'a> From<Theme> for Box<dyn iced::$name::StyleSheet + 'a> {
            fn from(theme: Theme) -> Self {
                match theme {
                    Theme::Light => light::$stylename.into(),
                    Theme::Dark => dark::$stylename.into(),
                }
            }
        }
    }
}

impl_from_lt!(container, Container);
// impl_from_lt!(container, ToolbarControls);