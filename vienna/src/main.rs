use eframe::{egui, App, Frame};
use std::{fs, path::PathBuf};

struct ViennaApp {
    tabs: Vec<(String, String, bool, Option<PathBuf>)>,
    current_tab: Option<usize>,
    show_close_buttons: bool,
}

impl Default for ViennaApp {
    fn default() -> Self {
        Self {
            tabs: vec![],
            current_tab: None,
            show_close_buttons: false,
        }
    }
}

impl App for ViennaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let bottom_bar_height = 25.0;

        egui::TopBottomPanel::bottom("bottom_bar")
            .resizable(false)
            .exact_height(bottom_bar_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Status: Ready");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("Vienna IDE");
                    });
                });
            });

        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            let tab_bar_height = 40.0;

            let pointer_pos = ctx.input(|i| i.pointer.hover_pos());
            self.show_close_buttons = if let Some(pos) = pointer_pos {
                pos.y <= tab_bar_height
            } else {
                false
            };

            if self.tabs.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        egui::RichText::new("Drop a file here to start")
                            .size(20.0)
                            .color(egui::Color32::LIGHT_GRAY),
                    );
                });
            } else {
                let mut tab_to_remove: Option<usize> = None;

                ui.horizontal(|ui| {
                    ui.add_space(5.0);
                    for (index, (file_name, _, is_saved, _)) in self.tabs.iter().enumerate() {
                        let is_selected = self.current_tab == Some(index);
                        let display_name = if *is_saved {
                            file_name.clone()
                        } else {
                            format!("{} *", file_name)
                        };

                        ui.horizontal(|ui| {
                            let tab_button = ui.add(
                                egui::Button::new(egui::RichText::new(&display_name).size(16.0))
                                    .frame(false),
                            );

                            if tab_button.clicked() {
                                self.current_tab = Some(index);
                            }

                            if self.show_close_buttons {
                                if ui.button("x").on_hover_text("Close").clicked() {
                                    tab_to_remove = Some(index);
                                }
                            }
                        });
                    }
                });

                ui.add_space(0.0);
                ui.separator();

                if let Some(index) = tab_to_remove {
                    self.tabs.remove(index);
                    if self.current_tab == Some(index) {
                        self.current_tab = None;
                    } else if let Some(current) = self.current_tab {
                        if current > index {
                            self.current_tab = Some(current - 1);
                        }
                    }
                }

                if let Some(current) = self.current_tab {
                    if let Some((_, content, is_saved, _)) = self.tabs.get_mut(current) {
                        egui::ScrollArea::both()
                            .auto_shrink([false; 2])
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(content)
                                        .frame(false)
                                        .desired_width(f32::INFINITY),
                                )
                            });
                    }
                }
            }

            if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
                for file in ctx.input(|i| i.raw.dropped_files.clone()) {
                    if let Some(path) = file.path {
                        if let Ok(content) = fs::read_to_string(&path) {
                            self.tabs.push((
                                path.file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .into_owned(),
                                content,
                                true,
                                Some(path),
                            ));
                            self.current_tab = Some(self.tabs.len() - 1);
                        }
                    }
                }
            }

            if ctx.input(|i| i.key_pressed(egui::Key::S)) && ctx.input(|i| i.modifiers.command) {
                if let Some(current) = self.current_tab {
                    if let Some((_, content, is_saved, file_path)) = self.tabs.get_mut(current) {
                        if let Some(path) = file_path {
                            if fs::write(path, content).is_ok() {
                                *is_saved = true;
                            }
                        }
                    }
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::Vec2::new(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Vienna IDE",
        options,
        Box::new(|_cc| Box::new(ViennaApp::default())),
    )
}