use crate::app::{AppUpdate, ViewState};
use iced::{button, Button, Row, Text};

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
        Row::new().push(
            Button::new(&mut self.to_editor_button_state, Text::new("Go to Editor"))
                .on_press(AppUpdate::ViewStateChanged(ViewState::Editor)),
        )
    }
}

// Styling

use crate::{app::theme::colors, class};
