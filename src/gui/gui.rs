extern crate crossbeam;
extern crate eframe;

use std::time::Duration;

use crossbeam::channel::{self, Receiver};
use eframe::egui::{self, Response, Ui};

use crate::{
    components::constants::{
        DEFUALT_COM_PORT, DEFUALT_STARTING_POSITION, HUGE_PADDING, LARGE_PADDING, MEDIUM_PADDING,
        NINETY_DEGREES, SMALL_PADDING,
    },
    gui::test_windows::navcon::qtp1::generate_navcon_qtp_1_maze,
    subsystems::system::{run_system, Mode},
};

use super::window_stack::{Window, WindowHistory};

enum QTPState {
    Busy,
    Idle,
}

pub struct MARVApp {
    state: WindowHistory,
    qtp_state: QTPState,
    sensor_positions_receiver: Receiver<[(f32, f32); 5]>,
}

impl MARVApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let (_, sens_rx) = channel::bounded(10);

        Self {
            state: WindowHistory::new(),
            qtp_state: QTPState::Idle,
            sensor_positions_receiver: sens_rx,
        }
    }

    fn paint_main_window(&mut self, ui: &mut Ui) {
        ui.heading("Welcome to the EPR 320 developmental test kit!");
        ui.add_space(LARGE_PADDING);

        // SNC tests
        ui.group(|ui| {
            ui.heading("SNC Tests");

            ui.add_space(MEDIUM_PADDING);

            ui.label("SNC");
            ui.horizontal(|ui| {
                if ui.button("QTP1").clicked() {}
                if ui.button("QTP2").clicked() {}
                if ui.button("QTP3").clicked() {}
            });

            ui.add_space(SMALL_PADDING);

            ui.label("NAVCON");
            ui.horizontal(|ui| {
                if ui.button("QTP1").clicked() {
                    self.state.push(Window::NAVCONQtp1);
                }
                if ui.button("QTP2").clicked() {}
                if ui.button("QTP3").clicked() {}
                if ui.button("QTP4").clicked() {}
                if ui.button("QTP5").clicked() {}
            });
        });

        ui.add_space(HUGE_PADDING);

        // SS tests
        ui.group(|ui| {
            ui.heading("SS Tests");

            ui.add_space(MEDIUM_PADDING);

            ui.horizontal(|ui| {
                if ui.button("QTP1").clicked() {}
                if ui.button("QTP2").clicked() {}
                if ui.button("QTP3").clicked() {}
                if ui.button("QTP4").clicked() {}
                if ui.button("QTP5").clicked() {}
            });
        });

        ui.add_space(HUGE_PADDING);

        // MDPS tests
        ui.group(|ui| {
            ui.heading("MDPS Tests");

            ui.add_space(MEDIUM_PADDING);

            ui.horizontal(|ui| {
                if ui.button("QTP1").clicked() {}
                if ui.button("QTP2").clicked() {}
                if ui.button("QTP3").clicked() {}
                if ui.button("QTP4").clicked() {}
                if ui.button("QTP5").clicked() {}
            });
        });

        ui.add_space(HUGE_PADDING);

        // Integration tests
        ui.group(|ui| {
            ui.heading("Integration Tests");

            ui.add_space(MEDIUM_PADDING);

            // placeholders
            ui.horizontal(|ui| {
                if ui.button("QTP1").clicked() {}
                if ui.button("QTP2").clicked() {}
                if ui.button("QTP3").clicked() {}
                if ui.button("QTP4").clicked() {}
                if ui.button("QTP5").clicked() {}
            });
        });
    }

    fn paint_navcon_qtp_1_window(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        if ui.button("back").clicked() {
            self.state.pop();
        }

        ui.add_space(LARGE_PADDING);

        ui.heading("    NAVCON QTP 1");

        match self.qtp_state {
            QTPState::Busy => {
                for positions in &self.sensor_positions_receiver {
                    println!("painting with: {:?}", positions);
                    generate_navcon_qtp_1_maze(ui, positions);
                    ctx.request_repaint();

                    if positions[0].1 > 1000.0 {
                        break;
                    }
                }
            }
            QTPState::Idle => {
                let maze = generate_navcon_qtp_1_maze(ui, [(0.1, 0.05); 5]);

                let (sens_tx, sens_rx) = channel::bounded(10);
                self.sensor_positions_receiver = sens_rx;

                if ui.button("start").clicked() {
                    self.qtp_state = QTPState::Busy;

                    std::thread::spawn(move || {
                        std::thread::sleep(Duration::from_millis(5));

                        run_system(
                            Mode::Emulate,
                            Mode::Emulate,
                            Mode::Emulate,
                            DEFUALT_COM_PORT,
                            DEFUALT_COM_PORT,
                            DEFUALT_COM_PORT,
                            maze,
                            DEFUALT_STARTING_POSITION, // in meters
                            NINETY_DEGREES,
                            sens_tx,
                        );
                    });
                }
            }
        }
    }
}

impl eframe::App for MARVApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(window) = self.state.curr_window() {
                match window {
                    Window::Main => self.paint_main_window(ui),
                    Window::NAVCONQtp1 => self.paint_navcon_qtp_1_window(ui, ctx),
                }
            } else {
                self.state.push(Window::Main);
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
