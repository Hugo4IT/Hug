use iced::{container, Column, Container, Length, Row, Text};

#[derive(Debug, Clone, Copy)]
pub enum EditorUpdate {}

#[derive(Default)]
pub struct EditorView {}

impl EditorView {
    pub fn new() -> Self {
        EditorView {}
    }

    pub fn update(&mut self, message: EditorUpdate) {}

    pub fn view<'a>(&mut self) -> Row<'a, AppUpdate> {
        Row::new()
            .push(
                Column::new()
                    .push(
                        Row::new()
                            .push(
                                Container::new(Text::new("Top left"))
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
            .into()
    }
}

// Styling

use crate::{
    app::{theme::colors, AppUpdate},
    class,
};

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
