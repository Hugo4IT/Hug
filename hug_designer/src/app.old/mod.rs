use iced::{
    alignment::{Horizontal, Vertical},
    button, container, Alignment, Button, Color, Column, Container, Element, Length, Row, Sandbox,
    Settings, Text, Vector,
};
use iced_native::Padding;

pub mod theme;
mod widgets;

#[derive(Debug, Clone, Copy)]
pub enum AppUpdate {
    SetAppState(AppState),
}

#[derive(Debug, Clone, Copy)]
pub enum AppState {
    ProjectList {
        open_editor_button: button::State,
    },
    Editor {
        back_to_project_list_button: button::State,
    },
}

impl AppState {
    pub fn new_project_list() -> AppState {
        AppState::ProjectList {
            open_editor_button: button::State::new(),
        }
    }

    pub fn new_editor() -> AppState {
        AppState::Editor {
            back_to_project_list_button: button::State::new(),
        }
    }
}

struct App {
    state: AppState,
}

impl Sandbox for App {
    type Message = AppUpdate;

    fn new() -> App {
        App {
            state: AppState::new_project_list(),
        }
    }

    fn title(&self) -> String {
        String::from("Hug Designer")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppUpdate::SetAppState(s) => self.state = s,
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content: Element<_> = match &mut self.state {
            AppState::ProjectList { open_editor_button } => Row::new()
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
                                            open_editor_button,
                                            Text::new("Go to Editor")
                                                .vertical_alignment(Vertical::Center)
                                                .horizontal_alignment(Horizontal::Center)
                                                .height(Length::Fill)
                                                .width(Length::Fill)
                                                .size(32),
                                        )
                                        .on_press(AppUpdate::SetAppState(AppState::new_editor()))
                                        .height(Length::Units(64))
                                        .width(Length::Fill)
                                        .style(SidebarButton),
                                    ),
                            )
                            .align_y(Vertical::Center)
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .style(Sidebar),
                        ),
                )
                .push(
                    Column::new().height(Length::Fill).width(Length::Fill).push(
                        Container::new(Row::new())
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .style(Overview),
                    ),
                )
                .into(),
            AppState::Editor {
                back_to_project_list_button,
            } => Row::new()
                .push(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    Container::new(
                                        Row::with_children(vec![Button::new(
                                            back_to_project_list_button,
                                            Text::new("Back to projects"),
                                        )
                                        .on_press(AppUpdate::SetAppState(
                                            AppState::new_project_list(),
                                        ))
                                        .into()])
                                        .padding(Padding::new(16))
                                        .spacing(8)
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .align_items(Alignment::Center),
                                    ) // Top Left
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .style(ToolbarControls),
                                )
                                .width(Length::Units(300))
                                .height(Length::Units(64)),
                        )
                        .push(
                            Row::new()
                                .push(
                                    Container::new(Text::new("Bottom left"))
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .style(SidebarFileSystem),
                                )
                                .width(Length::Units(300))
                                .height(Length::Fill),
                        ),
                )
                .push(
                    Column::new()
                        .push(
                            Row::new()
                                .push(
                                    Container::new(Text::new("Top right"))
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .style(ToolbarTabs),
                                )
                                .width(Length::Fill)
                                .height(Length::Units(64)),
                        )
                        .push(
                            Row::new()
                                .push(
                                    Container::new(Text::new("Bottom right"))
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .style(Viewport),
                                )
                                .width(Length::Fill)
                                .height(Length::Fill),
                        ),
                )
                .into(),
        };

        #[cfg(feature = "explain")]
        let content = content.explain(crate::app::theme::colors::GRAY[9]);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

use crate::{app::theme::colors, class};

class!(container ToolbarControls: container::Style {
    background: colors::GRAY[4].into(),
    text_color: colors::GRAY[9].into(),
    ..Default::default()
});

class!(container ToolbarTabs: container::Style {
    background: colors::GRAY[8].into(),
    text_color: colors::GRAY[0].into(),
    ..Default::default()
});

class!(container SidebarFileSystem: container::Style {
    background: colors::GRAY[7].into(),
    text_color: colors::GRAY[0].into(),
    ..Default::default()
});

class!(container Viewport: container::Style {
    background: colors::GRAY[9].into(),
    text_color: colors::GRAY[0].into(),
    ..Default::default()
});

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
            shadow_offset: Vector::new(0.0, 0.0),
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
            background: colors::GRAY[8].into(),
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

pub(super) fn init() {
    App::run(Settings::default()).unwrap();
}
