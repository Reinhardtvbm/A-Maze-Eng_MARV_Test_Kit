use crate::{
    components::adjacent_bytes::AdjacentBytes, subsystems::motor_subsystem::wheel::Wheels,
};
use std::sync::Arc;

use crate::components::{
    buffer::{BufferUser, SharedBuffer},
    comm_port::ControlByte,
    packet::Packet,
    state::SystemState,
};

/**
    # Motor-driver and Power Subsystem (MDPS) struct
    Provides a way to emulate the MDPS
**/
#[derive(Debug)]
pub struct Mdps {
    /// A shared buffer of type Rc<RefCell<_>>
    /// which is written to by the other two subsystems
    in_buffer: SharedBuffer,
    /// The shared buffers of the other two subsystems
    /// for the MDPS to send its data to
    out_buffer: SharedBuffer,
    /// Wheels is a struct that contains data for speed, distance,
    /// and rotation. It facilitates some of the calculations
    /// required to keep track of this data in autonomous mode
    wheels: Wheels,
    /// The state that the full is in
    state: SystemState,
    /// The desired operating velocity during maze navigation
    operational_velocity: u8,
}

impl Mdps {
    /// Returns a new MDPS
    ///
    /// If the test kit is running with one or more real subsystems, commport will be Some(..), otherwise it
    /// must be None
    pub fn new(out_buffer: &SharedBuffer, in_buffer: &SharedBuffer) -> Self {
        Self {
            in_buffer: Arc::clone(in_buffer),
            out_buffer: Arc::clone(out_buffer),
            wheels: Wheels::new(8.0),
            state: SystemState::Idle,
            operational_velocity: 0,
        }
    }

    pub fn run(&mut self) {
        match self.state {
            SystemState::Idle => {
                /* Idle things */

                if let Some(packet) = self.read() {
                    // if the control byte is correct, and a touch has been sensed
                    if packet.control_byte() == ControlByte::IdleButton && packet.dat1() == 1 {
                        self.operational_velocity = packet.dat0();
                        self.state = SystemState::Calibrate;
                    }
                }
            }
            SystemState::Calibrate => {
                /* Calibration things */

                if let Some(packet) = self.read() {
                    match packet.control_byte() {
                        ControlByte::Calibrated => {
                            self.write([
                                u8::from(ControlByte::CalibrateOperationalVelocity),
                                self.operational_velocity,
                                self.operational_velocity,
                                0,
                            ]);

                            self.write([u8::from(ControlByte::CalibrateBatteryLevel), 0, 0, 0]);
                        }
                        ControlByte::CalibrateButton => {
                            if packet.dat1() == 1 {
                                self.state = SystemState::Maze;
                            }
                        }
                        _ => (),
                    }
                }
            }
            SystemState::Maze => {
                /* Maze things */
                if let Some(packet) = self.read() {
                    match packet.control_byte() {
                        ControlByte::MazeClapSnap => {
                            if packet.dat1() == 1 {
                                self.state = SystemState::Sos;
                            }
                        }
                        ControlByte::MazeButton => {
                            if packet.dat1() == 1 {
                                self.state = SystemState::Idle;
                            }
                        }
                        ControlByte::MazeNavInstructions => {
                            let mut rotating = false;

                            match packet.dec() {
                                0 => {
                                    self.wheels
                                        .set_left_wheel_speed(self.operational_velocity as i16);
                                    self.wheels
                                        .set_right_wheel_speed(self.operational_velocity as i16);
                                }
                                1 => {
                                    self.wheels
                                        .set_left_wheel_speed(-(self.operational_velocity as i16));
                                    self.wheels
                                        .set_right_wheel_speed(-(self.operational_velocity as i16));
                                }
                                2 => {
                                    self.wheels
                                        .set_left_wheel_speed(-(self.operational_velocity as i16));
                                    self.wheels
                                        .set_right_wheel_speed(self.operational_velocity as i16);

                                    rotating = true;
                                }
                                3 => {
                                    self.wheels
                                        .set_left_wheel_speed(self.operational_velocity as i16);
                                    self.wheels
                                        .set_right_wheel_speed(-(self.operational_velocity as i16));

                                    rotating = true;
                                }
                                _ => (),
                            };

                            self.wheels.update_distance();

                            if rotating {
                                let target_rotation =
                                    AdjacentBytes::make(packet.dat1(), packet.dat0()).into();

                                while self.wheels.get_rotation() < target_rotation {
                                    self.wheels.update_distance();
                                }
                            }

                            // write battery level (no longer required as of 2022, so just send 0's)
                            self.write([161, 0, 0, 0]);

                            // get final rotation
                            let curr_rotation = self.wheels.get_rotation();

                            let rotation_bytes = AdjacentBytes::from(curr_rotation);

                            self.write([
                                162,
                                rotation_bytes.get_lsb(),
                                rotation_bytes.get_msb(),
                                match self.wheels.left_rotation() {
                                    true => 2,
                                    false => 3,
                                },
                            ]);

                            // write speed
                            self.write([
                                163,
                                self.wheels.get_left_wheel_speed(),
                                self.wheels.get_right_wheel_speed(),
                                match self.wheels.going_forward() {
                                    true => 0,
                                    false => 1,
                                },
                            ]);

                            // write distance
                            let distance = self.wheels.get_distance();

                            let distance_bytes = AdjacentBytes::from(distance);

                            self.write([
                                164,
                                distance_bytes.get_lsb(),
                                distance_bytes.get_msb(),
                                0,
                            ]);
                        }
                        ControlByte::MazeEndOfMaze => self.state = SystemState::Idle,
                        _ => (),
                    }
                }
            }
            SystemState::Sos => {
                /* SOS things */
                self.wheels.set_left_wheel_speed(0);
                self.wheels.set_right_wheel_speed(0);

                self.write([u8::from(ControlByte::SosSpeed), 0, 0, 0]);

                if let Some(packet) = self.read() {
                    if packet.control_byte() == ControlByte::SosClapSnap && packet.dat1() == 1 {
                        self.state = SystemState::Maze;
                    }
                }
            }
        }
    }
}

impl BufferUser for Mdps {
    fn write(&mut self, data: [u8; 4]) {
        self.out_buffer.lock().unwrap().write(data.into());
    }

    fn read(&mut self) -> Option<Packet> {
        self.in_buffer.lock().unwrap().read()
    }

    fn wait_for_packet(&mut self, control_byte: ControlByte) -> Packet {
        todo!()
    }
}
