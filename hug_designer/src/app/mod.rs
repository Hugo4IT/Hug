use iced::{container, Column, Container, Element, Length, Row, Sandbox, Settings, Text};

use self::theme::{colors, light};
mod theme;
mod widgets;

#[derive(Debug, Clone, Copy)]
pub enum AppUpdate {
    NumberChanged(i32),
    ThemeChanged(theme::Theme),
}

#[derive(Default)]
struct App {
    theme: theme::Theme,
    number: i32,
}

impl Sandbox for App {
    type Message = AppUpdate;

    fn new() -> App {
        App {
            theme: theme::Theme::default(),
            number: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Hug Designer")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppUpdate::NumberChanged(num) => self.number = num,
            AppUpdate::ThemeChanged(theme) => self.theme = theme,
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content: Element<_> = Row::new()
            .push(
                Column::new()
                    .push(
                        Row::new()
                            .push(Text::new("Top left"))
                            .width(Length::Units(300))
                            .height(Length::Units(64)),
                    )
                    .push(
                        Row::new()
                            .push(Text::new("Bottom left"))
                            .width(Length::Units(300))
                            .height(Length::Fill),
                    ),
            )
            .push(
                Column::new()
                    .push(
                        Row::new()
                            .push(
                                Container::new(Text::new("Bottom right"))
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .style(light::ToolbarControls)
                            )
                                .width(Length::Fill)
                                .height(Length::Units(64)),
                    )
                    .push(
                        Row::new()
                                .push(
                                    Container::new(Column::new())
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .style(light::ToolbarControls)
                                )
                                .width(Length::Fill)
                                .height(Length::Fill),
                    )
                )
                    .into();
        let content = content.explain(colors::GRAY[9]);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(self.theme)
            .into()
    }
}

pub(super) fn init() {
    App::run(Settings::default()).unwrap();
}
