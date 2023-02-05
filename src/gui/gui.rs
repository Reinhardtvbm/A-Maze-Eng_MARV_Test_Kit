extern crate crossbeam;
extern crate eframe;

use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use eframe::egui::{self, Response, Ui};

use crate::{
    asynchronous::async_type::PositionsEndpoint,
    components::{
        buffer::Buffer,
        colour::Colour,
        constants::{
            DEFUALT_COM_PORT, DEFUALT_STARTING_POSITION, HUGE_PADDING, LARGE_PADDING,
            MEDIUM_PADDING, NINETY_DEGREES, SMALL_PADDING,
        },
    },
    gui::test_windows::navcon::qtp1::generate_navcon_qtp_1_maze,
    gui::test_windows::navcon::qtp2::generate_navcon_qtp_2_maze,
    subsystems::system::{run_system, Mode},
};

use super::{
    test_windows::navcon::{
        qtp3::generate_navcon_qtp_3_maze, qtp4::generate_navcon_qtp_4_maze,
        qtp5::generate_navcon_qtp_5_maze,
    },
    window_stack::{QtpNo, Window, WindowHistory},
};

enum QTPState {
    Busy,
    Idle,
}

pub struct MARVApp {
    state: WindowHistory,
    qtp_state: QTPState,
    sensor_positions: PositionsEndpoint,
    test_thread: Option<JoinHandle<()>>,
    com_no: Option<String>,
}

impl MARVApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            state: WindowHistory::new(),
            qtp_state: QTPState::Idle,
            sensor_positions: Arc::new(Mutex::new(Buffer::new())),
            test_thread: None,
            com_no: None,
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
                    self.state.push(Window::Navcon(QtpNo::Qtp1));
                }
                if ui.button("QTP2").clicked() {
                    self.state.push(Window::Navcon(QtpNo::Qtp2));
                }
                if ui.button("QTP3").clicked() {
                    self.state.push(Window::Navcon(QtpNo::Qtp3));
                }
                if ui.button("QTP4").clicked() {
                    self.state.push(Window::Navcon(QtpNo::Qtp4));
                }
                if ui.button("QTP5").clicked() {
                    self.state.push(Window::Navcon(QtpNo::Qtp5));
                }
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

    fn paint_navcon_qtp_window(&mut self, ui: &mut Ui, ctx: &egui::Context, qtp_no: QtpNo) {
        if ui.button("back").clicked() {
            self.state.pop();
        }

        ui.add_space(LARGE_PADDING);

        // add the correct heading to the Window
        match qtp_no {
            QtpNo::Qtp1 => {
                ui.heading("    NAVCON QTP 1");
            }
            QtpNo::Qtp2 => {
                ui.heading("    NAVCON QTP 2");
            }
            QtpNo::Qtp3 => {
                ui.heading("    NAVCON QTP 3");
            }
            QtpNo::Qtp4 => {
                ui.heading("    NAVCON QTP 4");
            }
            QtpNo::Qtp5 => {
                ui.heading("    NAVCON QTP 5");
            }
        }

        ui.add_space(LARGE_PADDING);

        // =================================================================================
        // WINDOW PROCESSING:

        match self.qtp_state {
            QTPState::Idle => {
                // generate the MazeLineMap based on qtp number
                let maze = match qtp_no {
                    QtpNo::Qtp1 => generate_navcon_qtp_1_maze(ui, [(0.1, 0.05); 5]),
                    QtpNo::Qtp2 => generate_navcon_qtp_2_maze(ui, [(0.1, 0.05); 5]),
                    QtpNo::Qtp3 => generate_navcon_qtp_3_maze(ui, [(0.1, 0.05); 5]),
                    QtpNo::Qtp4 => generate_navcon_qtp_4_maze(ui, [(0.1, 0.05); 5]),
                    QtpNo::Qtp5 => generate_navcon_qtp_5_maze(ui, [(0.1, 0.05); 5]),
                };

                ui.horizontal(|ui| {
                    if ui.button("start").clicked() {
                        self.qtp_state = QTPState::Busy;

                        let gui_thread_origin = Arc::clone(&self.sensor_positions);

                        self.test_thread = Some(std::thread::spawn(move || {
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
                                &gui_thread_origin,
                            );
                        }));
                    }

                    ui.add_space(MEDIUM_PADDING);

                    ui.menu_button("Select COM Port", |ui| {
                        for port in serialport::available_ports().unwrap() {
                            if ui
                                .button(format!("{} ({:?})", port.port_name, port.port_type))
                                .clicked()
                            {
                                self.com_no = Some(port.port_name);
                            }
                        }
                    });
                });
            }
            QTPState::Busy => {
                if let Some(positions) = self.sensor_positions.lock().unwrap().read() {
                    println!("painting with: {:?}", positions);

                    let maze = match qtp_no {
                        QtpNo::Qtp1 => generate_navcon_qtp_1_maze(ui, positions),
                        QtpNo::Qtp2 => generate_navcon_qtp_2_maze(ui, positions),
                        QtpNo::Qtp3 => generate_navcon_qtp_3_maze(ui, positions),
                        QtpNo::Qtp4 => generate_navcon_qtp_4_maze(ui, positions),
                        QtpNo::Qtp5 => generate_navcon_qtp_5_maze(ui, positions),
                    };

                    let colours = maze.get_colours(positions);

                    if colours.iter().all(|colour| *colour == Colour::Red) {
                        self.qtp_state = QTPState::Idle;
                    }

                    ctx.request_repaint();
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
                    Window::Navcon(qtp_no) => self.paint_navcon_qtp_window(ui, ctx, qtp_no),
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
