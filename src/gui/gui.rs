extern crate crossbeam;
extern crate eframe;

use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
    thread::JoinHandle,
    time::Duration,
};

use eframe::egui::{self, Response, Ui};

use crate::{
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
    test_windows::navcon::qtp3::generate_navcon_qtp_3_maze,
    window_stack::{Window, WindowHistory},
};

enum QTPState {
    Busy,
    Idle,
}

pub struct MARVApp {
    state: WindowHistory,
    qtp_state: QTPState,
    sensor_positions: Arc<Mutex<Buffer<[(f32, f32); 5]>>>,
    test_thread: Option<JoinHandle<()>>,
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
                if ui.button("QTP2").clicked() {
                    self.state.push(Window::NAVCONQtp2);
                }
                if ui.button("QTP3").clicked() {
                    self.state.push(Window::NAVCONQtp3);
                }
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
                if let Some(positions) = self.sensor_positions.lock().unwrap().read() {
                    println!("painting with: {:?}", positions);
                    let maze = generate_navcon_qtp_1_maze(ui, positions);

                    let mut colours = Vec::new();

                    // get the colours under each sensor
                    positions.iter().for_each(|sensor_pos| {
                        colours.push(
                            maze.get_colour_from_coord(sensor_pos.0, sensor_pos.1)
                                .expect("FATAL: colour in maze not found"),
                        )
                    });

                    if positions[0].1 > 1000.0
                        || colours.iter().all(|colour| *colour == Colour::Red)
                    {
                        self.qtp_state = QTPState::Idle;
                    }

                    ctx.request_repaint();
                }
            }
            QTPState::Idle => {
                let maze = generate_navcon_qtp_1_maze(ui, [(0.1, 0.05); 5]);

                if ui.button("start").clicked() {
                    self.qtp_state = QTPState::Busy;

                    let gui_thread_origin = Arc::clone(&self.sensor_positions);

                    self.test_thread = Some(std::thread::spawn(move || {
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
                            &gui_thread_origin,
                        );
                    }));
                }
            }
        }
    }

    fn paint_navcon_qtp_2_window(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        if ui.button("back").clicked() {
            self.state.pop();
        }

        ui.add_space(LARGE_PADDING);

        ui.heading("    NAVCON QTP 2");

        match self.qtp_state {
            QTPState::Busy => {
                if let Some(positions) = self.sensor_positions.lock().unwrap().read() {
                    println!("painting with: {:?}", positions);
                    let maze = generate_navcon_qtp_2_maze(ui, positions);

                    let mut colours = Vec::new();

                    // get the colours under each sensor
                    positions.iter().for_each(|sensor_pos| {
                        colours.push(
                            maze.get_colour_from_coord(sensor_pos.0, sensor_pos.1)
                                .expect("FATAL: colour in maze not found"),
                        )
                    });

                    if positions[0].1 > 1000.0
                        || colours.iter().all(|colour| *colour == Colour::Red)
                    {
                        self.qtp_state = QTPState::Idle;
                    }

                    ctx.request_repaint();
                }
            }
            QTPState::Idle => {
                let maze = generate_navcon_qtp_2_maze(ui, [(0.1, 0.05); 5]);

                if ui.button("start").clicked() {
                    self.qtp_state = QTPState::Busy;

                    let gui_thread_origin = Arc::clone(&self.sensor_positions);

                    self.test_thread = Some(std::thread::spawn(move || {
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
                            &gui_thread_origin,
                        );
                    }));
                }
            }
        }
    }

    fn paint_navcon_qtp_3_window(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        if ui.button("back").clicked() {
            self.state.pop();
        }

        ui.add_space(LARGE_PADDING);

        ui.heading("    NAVCON QTP 3");

        match self.qtp_state {
            QTPState::Busy => {
                if let Some(positions) = self.sensor_positions.lock().unwrap().read() {
                    println!("painting with: {:?}", positions);
                    let maze = generate_navcon_qtp_3_maze(ui, positions);

                    let mut colours = Vec::new();

                    // get the colours under each sensor
                    positions.iter().for_each(|sensor_pos| {
                        colours.push(
                            maze.get_colour_from_coord(sensor_pos.0, sensor_pos.1)
                                .expect("FATAL: colour in maze not found"),
                        )
                    });

                    if positions[0].1 > 1000.0
                        || colours.iter().all(|colour| *colour == Colour::Red)
                    {
                        self.qtp_state = QTPState::Idle;
                    }

                    ctx.request_repaint();
                }
            }
            QTPState::Idle => {
                let maze = generate_navcon_qtp_3_maze(ui, [(0.1, 0.05); 5]);

                if ui.button("start").clicked() {
                    self.qtp_state = QTPState::Busy;

                    let gui_thread_origin = Arc::clone(&self.sensor_positions);

                    self.test_thread = Some(std::thread::spawn(move || {
                        std::thread::sleep(Duration::from_millis(5));

                        run_system(
                            Mode::Emulate,
                            Mode::Emulate,
                            Mode::Emulate,
                            DEFUALT_COM_PORT,
                            DEFUALT_COM_PORT,
                            DEFUALT_COM_PORT,
                            maze,
                            (
                                DEFUALT_STARTING_POSITION.0 - 0.02,
                                DEFUALT_STARTING_POSITION.1,
                            ), // in meters
                            NINETY_DEGREES - (PI / 9.0),
                            &gui_thread_origin,
                        );
                    }));
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
                    Window::NAVCONQtp2 => self.paint_navcon_qtp_2_window(ui, ctx),
                    Window::NAVCONQtp3 => self.paint_navcon_qtp_3_window(ui, ctx),
                    Window::NAVCONQtp4 => todo!(),
                    Window::NAVCONQtp5 => todo!(),
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
