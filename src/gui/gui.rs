use eframe::egui::{self, Response, Ui};

use super::{
    entry_window::EntryWindow,
    window_stack::{Window, WindowHistory},
};

pub struct MARVApp {
    state: WindowHistory,
    entry_window: EntryWindow,
}

impl MARVApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            state: WindowHistory::new(),
            entry_window: EntryWindow::new(),
        }
    }

    fn paint_entry_window(&mut self, ui: &mut Ui) {
        ui.heading("Sybsystem modes");
        ui.separator();
        ui.horizontal(|inner| {
            let snc_button_str = format!("SNC: {}", self.entry_window.snc_mode());
            let ss_button_str = format!("SS: {}", self.entry_window.ss_mode());
            let mdps_button_str = format!("MDPS: {}", self.entry_window.mdps_mode());

            if inner.button(snc_button_str).clicked() {
                self.entry_window.toggle_snc_mode();
            }

            if inner.button(ss_button_str).clicked() {
                self.entry_window.toggle_ss_mode();
            }

            if inner.button(mdps_button_str).clicked() {
                self.entry_window.toggle_mdps_mode();
            }
        });

        if ui.button("Done").clicked() {
            self.state.push(Window::TestSelect);
        }
    }

    fn paint_maze_select_window(&mut self, ui: &mut Ui) {
        if ui.button("back").clicked() {
            self.state.pop();
        }

        ui.heading("Maze Selection");

        ui.horizontal(|ui| {
            let maze_1_button = ui.button("Maze 1");
            let _placeholder_1 = ui.button("Maze 2");
            let _placeholder_2 = ui.button(" ...  ");

            if maze_1_button.clicked() {
                self.state.push(Window::Maze1);
            }
        });
    }

    fn paint_maze_1_window(&mut self, ui: &mut Ui) {
        if ui.button("back").clicked() {
            self.state.pop();
        }

        ui.heading("Maze 1");
    }
}

impl eframe::App for MARVApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(window) = self.state.curr_window() {
                match window {
                    Window::Entry => self.paint_entry_window(ui),
                    Window::TestSelect => self.paint_maze_select_window(ui),
                    Window::Maze1 => self.paint_maze_1_window(ui),
                }
            } else {
                self.state.push(Window::Entry);
            }
        });
    }
}

enum Event {
    Hovered,
    Clicked,
    DoubleClicked,
    TripleClicked,
    None,
}

trait Match {
    fn get_event(&self) -> Event;
}

impl Match for Response {
    fn get_event(&self) -> Event {
        if self.hovered() {
            Event::Hovered
        } else if self.clicked() {
            Event::Clicked
        } else if self.double_clicked() {
            Event::DoubleClicked
        } else if self.triple_clicked() {
            Event::TripleClicked
        } else {
            Event::None
        }
    }
}
