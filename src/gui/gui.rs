extern crate crossbeam;
extern crate eframe;

use std::{
    collections::VecDeque,
    f32::consts::PI,
    sync::{Arc, Mutex},
    time::Duration,
};

use crossbeam::channel::{self, Receiver, Sender};
use eframe::egui::{self, Response, Ui};

use crate::subsystems::system::{run_system, Mode};

use super::{
    test_windows::navcon::qtp1::paint_navcon_qtp_1,
    window_stack::{Window, WindowHistory},
};

enum QTPState {
    Busy,
    Idle,
}

pub struct MARVApp {
    state: WindowHistory,
    qtp_state: QTPState,
    sensor_positions_receiver: Receiver<[(f32, f32); 5]>,
    sensor_positions_sender: Sender<[(f32, f32); 5]>,
    prev_sensor_positions: [(f32, f32); 5],
}

impl MARVApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let (sens_tx, sens_rx) = channel::bounded(10);

        Self {
            state: WindowHistory::new(),
            qtp_state: QTPState::Idle,
            sensor_positions_receiver: sens_rx,
            sensor_positions_sender: sens_tx,
            prev_sensor_positions: [(0., 0.); 5],
        }
    }

    fn paint_main_window(&mut self, ui: &mut Ui) {
        ui.heading("Welcome to the EPR 320 developmental test kit!");
        ui.add_space(8.0);

        // SNC tests
        ui.group(|ui| {
            ui.heading("SNC Tests");

            ui.add_space(4.0);

            ui.label("SNC");
            ui.horizontal(|ui| {
                if ui.button("QTP1").clicked() {}
                if ui.button("QTP2").clicked() {}
                if ui.button("QTP3").clicked() {}
            });

            ui.add_space(2.0);

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

        ui.add_space(12.0);

        // SS tests
        ui.group(|ui| {
            ui.heading("SS Tests");

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button("QTP1").clicked() {}
                if ui.button("QTP2").clicked() {}
                if ui.button("QTP3").clicked() {}
                if ui.button("QTP4").clicked() {}
                if ui.button("QTP5").clicked() {}
            });
        });

        ui.add_space(12.0);

        // MDPS tests
        ui.group(|ui| {
            ui.heading("MDPS Tests");

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button("QTP1").clicked() {}
                if ui.button("QTP2").clicked() {}
                if ui.button("QTP3").clicked() {}
                if ui.button("QTP4").clicked() {}
                if ui.button("QTP5").clicked() {}
            });
        });

        ui.add_space(12.0);

        // Integration tests
        ui.group(|ui| {
            ui.heading("Integration Tests");

            ui.add_space(4.0);

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

        ui.add_space(8.0);

        ui.heading("    NAVCON QTP 1");

        let (sens_tx, sens_rx) = channel::bounded(10);

        self.sensor_positions_receiver = sens_rx;
        self.sensor_positions_sender = sens_tx;

        match self.qtp_state {
            QTPState::Busy => {
                for positions in &self.sensor_positions_receiver {
                    paint_navcon_qtp_1(ui, positions);
                    ctx.request_repaint();
                }
            }
            QTPState::Idle => {
                let maze_map = paint_navcon_qtp_1(ui, [(0., 0.); 5]);

                if ui.button("start").clicked() {
                    self.qtp_state = QTPState::Busy;

                    std::thread::spawn(move || {
                        std::thread::sleep(Duration::from_millis(5));

                        run_system(
                            Mode::Emulate,
                            Mode::Emulate,
                            Mode::Emulate,
                            String::from('0'),
                            String::from('0'),
                            String::from('0'),
                            maze_map,
                            (0.1, 0.05), // in meters
                            PI / 2.0,
                            &self.sensor_positions_sender,
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
