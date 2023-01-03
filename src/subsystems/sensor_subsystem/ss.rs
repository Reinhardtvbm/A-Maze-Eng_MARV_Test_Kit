use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use crate::{
    components::{
        adjacent_bytes::AdjacentBytes,
        buffer::{BufferUser, SharedBuffer},
        colour::{Colour, Colours},
        comm_port::ControlByte,
        constants::{self, AXLE_DIST, B_ISD, S_ISD},
        packet::Packet,
        state::SystemState,
    },
    gui::maze::MazeLineMap,
};

#[derive(Debug)]
pub struct Ss {
    /// the x-y coordinates of each sensor, these will be
    /// updated using the distance and rotation measurements
    /// from the MDPS
    sensor_positions: [(f32, f32); 5],
    /// the colour that each sensor senses, will get via the
    /// `get_colour_from_coord` method in the `maze` module
    sensor_colours: Colours,
    /// A shared buffer of type Rc<RefCell<_>>
    /// which is written to by the other two subsystems
    in_buffer: SharedBuffer,
    /// The shared buffers of the other two subsystems
    /// for the MDPS to send its data to
    out_buffer: SharedBuffer,
    state: SystemState,
    wheel_info: Arc<Mutex<VecDeque<i16>>>,
}

impl Ss {
    pub fn new(
        out_buffer: &SharedBuffer,
        in_buffer: &SharedBuffer,
        init_sensor_pos: [(f32, f32); 5],
        wheel: &Arc<Mutex<VecDeque<i16>>>,
    ) -> Self {
        Self {
            sensor_colours: Colours::new(),
            sensor_positions: init_sensor_pos,
            in_buffer: Arc::clone(in_buffer),
            out_buffer: Arc::clone(out_buffer),
            state: SystemState::Idle,
            wheel_info: Arc::clone(wheel),
        }
    }

    pub fn run(&mut self, maze: &MazeLineMap) {
        let mut end_of_maze = false;

        let mut time = SystemTime::now();
        let (mut prev_right, mut prev_left) = (0, 0);
        let mut prev_angular_velocity = 0.;
        let mut angle: f64 = 0.;

        let inside_rad: f64 =
            ((AXLE_DIST as f64).powi(2) + (S_ISD as f64).powi(2)).sqrt() / 1_000.0;
        let outside_rad: f64 =
            ((AXLE_DIST as f64).powi(2) + (S_ISD as f64 + B_ISD as f64).powi(2)).sqrt() / 1_000.0;

        let sensor_rads: [f64; 5] = [
            outside_rad,
            inside_rad,
            AXLE_DIST as f64 / 1_000.0,
            inside_rad,
            outside_rad,
        ];

        while !end_of_maze {
            match self.state {
                SystemState::Idle => {
                    /* IDLE */

                    if let Some(packet) = self.read() {
                        // if the control byte is correct, and a touch has been sensed
                        if packet.control_byte() == ControlByte::IdleButton && packet.dat1() == 1 {
                            self.state = SystemState::Calibrate;
                            self.write([112, 0, 0, 0]);
                            self.wait_for_packet(96.into());
                        }
                    }
                }
                SystemState::Calibrate => {
                    /* CALIBRATE */

                    self.wait_for_packet(97.into());

                    self.write([113, 0, 0, 0]);

                    if self.wait_for_packet(80.into()).dat1() == 1 {
                        self.state = SystemState::Maze;
                    }

                    time = SystemTime::now();
                }
                SystemState::Maze => {
                    /* MAZE */
                    let left_speed_opt = self.wheel_info.lock().unwrap().pop_front();
                    let right_speed_opt = self.wheel_info.lock().unwrap().pop_front();

                    if left_speed_opt.is_some() && right_speed_opt.is_some() {
                        let left_speed = left_speed_opt.unwrap();
                        let right_speed = right_speed_opt.unwrap();

                        let elapsed_time = time.elapsed().unwrap().as_millis() as f64 / 1_000.0; // s
                        time = SystemTime::now();

                        let angular_velocity = (right_speed - left_speed) as f64
                            / (constants::AXLE_DIST as f64 / 1_000.0); // rad/s

                        // trapezoidal rule
                        angle += elapsed_time * ((prev_angular_velocity + angular_velocity) / 2.0);

                        // update sensor_positions
                        sensor_rads.iter().enumerate().for_each(|(index, rad)| {
                            self.sensor_positions[index].0 += ((*rad) * angle.cos()) as f32;
                            self.sensor_positions[index].1 += ((*rad) * angle.sin()) as f32;
                        });
                    }

                    let mut colours = Vec::new();

                    self.sensor_positions.iter().for_each(|sensor_pos| {
                        colours.push(
                            maze.get_colour_from_coord(sensor_pos.0, sensor_pos.1)
                                .expect("FATAL: colour in maze not found"),
                        )
                    });

                    //println!("sensor: {:?}", colours);

                    if colours.iter().all(|colour| *colour == Colour::Red) {
                        end_of_maze = true;
                    }

                    if self.wait_for_packet(145.into()).dat1() == 1 {
                        self.state = SystemState::Sos;
                        break;
                    }

                    if self.wait_for_packet(146.into()).dat1() == 1 {
                        self.state = SystemState::Idle;
                        break;
                    }

                    self.wait_for_packet(164.into());

                    if end_of_maze {
                        self.write([179, 0, 0, 0]);
                    } else {
                        let mut word: u16 = 0;

                        for (index, colour) in colours.into_iter().enumerate() {
                            word |= (colour as u16) << 12 >> (index * 3);
                        }

                        let bytes: AdjacentBytes = word.into();

                        self.write([177, bytes.get_msb(), bytes.get_lsb(), 0]);
                        self.write([178, 0, 0, 0]);
                    }
                }
                SystemState::Sos => todo!(),
            }
        }
    }
}

impl BufferUser for Ss {
    fn write(&mut self, data: [u8; 4]) {
        self.out_buffer.lock().unwrap().write(data.into());
    }

    fn read(&mut self) -> Option<Packet> {
        self.in_buffer.lock().unwrap().read()
    }

    fn wait_for_packet(&mut self, control_byte: ControlByte) -> Packet {
        let mut received_packet = None;

        while received_packet.is_none() {
            if let Some(in_packet) = self.read() {
                if in_packet.control_byte() == control_byte {
                    received_packet = Some(in_packet);
                }
            }
        }

        received_packet.unwrap()
    }
}
