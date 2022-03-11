use eframe::{egui, epi};
use node_browser::NodeBrowser;

// mod app;
mod node_browser;

#[derive(Debug, Default)]
pub struct Node {
    pub id: usize,
    pub title: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Default)]
pub struct App {
    nodes: Vec<Node>,
    last_id: usize,
    node_browser: NodeBrowser,
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Add Node").clicked() {
                self.last_id += 1;

                self.nodes.push(Node {
                    id: self.last_id,
                    title: String::from("Add"),
                    inputs: vec![String::from("Left"), String::from("Right")],
                    outputs: vec![String::from("Output")],
                })
            }
        });

        for node in self.nodes.iter() {
            egui::Window::new(node.title.as_str())
                .id(egui::Id::new(node.id))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            for input in node.inputs.iter() {
                                ui.label(input);
                            }
                        });
                        ui.add_space(32.0);
                        ui.vertical(|ui| {
                            for output in node.outputs.iter() {
                                ui.label(output);
                            }
                        });
                    });
                });
        }

        self.node_browser.update(ctx, frame);
    }

    fn name(&self) -> &str {
        "Hug Designer"
    }
}

fn main() {
    // app::init();
    eframe::run_native(Box::new(App::default()), eframe::NativeOptions::default());
}
