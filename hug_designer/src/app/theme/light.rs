use iced::{Color, container};

use super::colors;

pub struct Container;
impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(colors::GRAY[0].into()),
            text_color: Some(colors::GRAY[9].into()),
            ..Default::default()
        }
    }
}

pub struct ToolbarControls;
impl container::StyleSheet for ToolbarControls {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(colors::GRAY[2].into()),
            text_color: Some(colors::GRAY[9].into()),
            ..Default::default()
        }
    }
}