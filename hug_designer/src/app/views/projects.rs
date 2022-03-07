use crate::app::{AppUpdate, ViewState};
use iced::{button, Button, Row, Text, Container, Column, Length, alignment::{Vertical, Horizontal}, container, Color, Vector, Font};
use iced_native::Padding;

#[derive(Debug, Clone, Copy)]
pub enum ProjectUpdate {}

#[derive(Default)]
pub struct ProjectView {
    to_editor_button_state: button::State,
}

impl ProjectView {
    pub fn new() -> Self {
        ProjectView {
            to_editor_button_state: button::State::new(),
        }
    }

    pub fn update(&mut self, message: ProjectUpdate) {}

    pub fn view<'a>(&'a mut self) -> Row<'a, AppUpdate> {
        Row::new()
            .push(
                Column::new()
                    .height(Length::Fill)
                    .width(Length::Units(300))
                    .push(
                        Container::new(
                            Column::new()
                                .spacing(0)
                                .width(Length::Fill)
                                .height(Length::Shrink)
                                .padding(Padding::new(0))
                                .push(
                                    Button::new(
                                        &mut self.to_editor_button_state,
                                        Text::new("Go to Editor")
                                            .vertical_alignment(Vertical::Center)
                                            .horizontal_alignment(Horizontal::Center)
                                            .height(Length::Fill)
                                            .width(Length::Fill)
                                            .size(32)
                                    )
                                        .on_press(AppUpdate::ViewStateChanged(ViewState::Editor))
                                        .height(Length::Units(64))
                                        .width(Length::Fill)
                                        .style(SidebarButton),
                                )
                        )
                            .align_y(Vertical::Center)
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .style(Sidebar)
                    )
            )
            .push(
                Column::new()
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .push(
                        Container::new(
                            Row::new()
                        )
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .style(Overview)
                    )
            )
    }
}

// Styling

use crate::{app::theme::colors, class};

class!(container Sidebar: container::Style {
    text_color: colors::GRAY[0].into(),
    background: colors::GRAY[8].into(),
    ..Default::default()
});

class!(container Overview: container::Style {
    text_color: colors::GRAY[0].into(),
    background: colors::GRAY[9].into(),
    ..Default::default()
});

pub struct SidebarButton;

impl button::StyleSheet for SidebarButton {
    fn active(&self) -> button::Style {
        button::Style {
            text_color: colors::GRAY[0].into(),
            background: colors::GRAY[8].into(),
            border_color: Color::TRANSPARENT,
            border_radius: 0.0,
            border_width: 0.0,
            shadow_offset: Vector::new(0.0, 0.0)
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: colors::GRAY[7].into(),
            ..self.active()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: colors::GRAY[6].into(),
            ..self.active()
        }
    }

    fn disabled(&self) -> button::Style {
        let active = self.active();

        button::Style {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                iced::Background::Color(color) => iced::Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}