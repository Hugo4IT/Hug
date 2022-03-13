use eframe::{epi, egui};

#[derive(Debug)]
pub struct NodeBrowser {
    search_bar: String,
    position: egui::Pos2,
    set_focus: bool,
    is_open: bool,
}

impl NodeBrowser {
    pub fn new() -> NodeBrowser {
        NodeBrowser {
            search_bar: String::new(),
            position: egui::Pos2::ZERO,
            set_focus: false,
            is_open: false,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        if self.is_open {
            egui::Window::new("Node Browser")
                .title_bar(false)
                .fixed_pos(self.position)
                .show(ctx, |ui| {
                    let search_bar = ui.text_edit_singleline(&mut self.search_bar);
                    if self.set_focus {
                        search_bar.request_focus();
                        self.set_focus = false;
                    }
                });
        } else {
            if let Some(pos) = ctx.pointer_interact_pos() {
                self.position = pos;
            }

            if ctx.input().key_released(egui::Key::Space) {
                self.is_open = true;
                self.set_focus = true;
            }
        }
    }
}

impl Default for NodeBrowser {
    fn default() -> Self {
        Self::new()
    }
}