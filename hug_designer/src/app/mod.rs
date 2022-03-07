use iced::{Container, Element, Length, Sandbox, Settings};

use self::views::{
    editor::{EditorUpdate, EditorView},
    projects::{ProjectUpdate, ProjectView},
};

mod theme;
mod views;
mod widgets;

#[derive(Debug, Clone, Copy)]
pub enum AppUpdate {
    EditorUpdate(EditorUpdate),
    ProjectUpdate(ProjectUpdate),
    ViewStateChanged(ViewState),
}

#[derive(Debug, Clone, Copy)]
pub enum ViewState {
    Editor,
    Projects,
}

struct App {
    editor_view: EditorView,
    project_view: ProjectView,
    current_view: ViewState,
}

impl Sandbox for App {
    type Message = AppUpdate;

    fn new() -> App {
        App {
            editor_view: EditorView::new(),
            project_view: ProjectView::new(),
            current_view: ViewState::Projects,
        }
    }

    fn title(&self) -> String {
        String::from("Hug Designer")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppUpdate::EditorUpdate(e) => self.editor_view.update(e),
            AppUpdate::ProjectUpdate(p) => self.project_view.update(p),
            AppUpdate::ViewStateChanged(v) => self.current_view = v,
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content: Element<_> = match self.current_view {
            ViewState::Editor => self.editor_view.view(),
            ViewState::Projects => self.project_view.view(),
        }
        .into();

        #[cfg(feature = "explain")]
        let content = content.explain(crate::app::theme::colors::GRAY[9]);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

pub(super) fn init() {
    App::run(Settings::default()).unwrap();
}
