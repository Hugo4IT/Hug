use eframe::{epi, egui};

pub struct NodeDescriptor {

}

#[derive(Debug)]
pub struct NodeBrowser {
    search_bar: String,
    is_open: bool,
    position: egui::Pos2,
}

impl NodeBrowser {
    pub fn new() -> NodeBrowser {
        NodeBrowser {
            position: egui::Pos2::ZERO,
            search_bar: String::new(),
            is_open: false,
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        if self.is_open {
            egui::Window::new("Node Browser")
                .title_bar(false)
                .fixed_pos(self.position)
                .show(ctx, |ui| {
                    ui.text_edit_singleline(&mut self.search_bar);
                });
        } else {
            if let Some(pos) = ctx.pointer_interact_pos() {
                self.position = pos;
            }

            if { ctx.input().key_pressed(egui::Key::Space) }
            || { ctx.input().pointer.secondary_down() } {
                self.is_open = true;
            }
        }
    }


}

impl Default for NodeBrowser {
    fn default() -> Self {
        Self::new()
    }
}