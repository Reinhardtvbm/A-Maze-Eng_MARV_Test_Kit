use crate::{
    components::adjacent_bytes::AdjacentBytes, subsystems::motor_subsystem::wheel::Wheels,
};
use std::rc::Rc;

use crate::components::{
    buffer::{BufferUser, Get, SharedBuffer},
    comm_port::{ComPort, ControlByte},
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
    read_buffer: SharedBuffer,
    /// The shared buffers of the other two subsystems
    /// for the MDPS to send its data to
    write_buffers: [SharedBuffer; 2],
    /// The ComPort which the MDPS is connected to
    /// ability to run the system fully automatically
    /// is planned, to in this case, none of the subsystem
    /// structs will have a ComPort.
    /// `ComPort` is an abstraction for `SerialPort` from
    /// the Rust serialport crete
    port: Option<ComPort>,
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
    pub fn new(
        w_buffers: [&SharedBuffer; 2],
        r_buffer: &SharedBuffer,
        activate_port: bool,
    ) -> Self {
        let comm_port = match activate_port {
            true => Some(ComPort::new(String::from("69"), 19200)),
            false => None,
        };

        Self {
            read_buffer: Rc::clone(r_buffer),
            write_buffers: [Rc::clone(w_buffers[0]), Rc::clone(w_buffers[1])],
            port: comm_port,
            wheels: Wheels::new(8.0),
            state: SystemState::Idle,
            operational_velocity: 0,
        }
    }

    pub fn run(&mut self) {
        match self.state {
            SystemState::Idle =>
            /* Idle things */
            {
                if let Some(packet) = self.read() {
                    if packet.control_byte() == ControlByte::IdleButton && packet.dat1() == 1 {
                        self.operational_velocity = packet.dat0();
                        self.state = SystemState::Calibrate;
                    }
                }
            }
            SystemState::Calibrate =>
            /* Calibration things */
            {
                if let Some(packet) = self.read() {
                    match packet.control_byte() {
                        ControlByte::Calibrated => {
                            self.write(&mut [
                                u8::from(ControlByte::CalibrateOperationalVelocity),
                                self.operational_velocity,
                                self.operational_velocity,
                                0,
                            ]);

                            self.write(&mut [
                                u8::from(ControlByte::CalibrateBatteryLevel),
                                0,
                                0,
                                0,
                            ]);
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
                                }
                                3 => {
                                    self.wheels
                                        .set_left_wheel_speed(self.operational_velocity as i16);
                                    self.wheels
                                        .set_right_wheel_speed(-(self.operational_velocity as i16));
                                }
                                _ => (),
                            };

                            self.wheels.update_distance();

                            // write battery level (no longer required as of 2022. i.e just send 0's)
                            self.write(&mut [161, 0, 0, 0]);

                            // write rotation
                            let rotation = self.wheels.get_rotation();

                            let rotation_bytes = AdjacentBytes::from(rotation);

                            self.write(&mut [
                                162,
                                rotation_bytes.get_lsb(),
                                rotation_bytes.get_msb(),
                                match self.wheels.left_rotation() {
                                    true => 2,
                                    false => 3,
                                },
                            ]);

                            // write speed
                            self.write(&mut [
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

                            self.write(&mut [
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

                self.write(&mut [u8::from(ControlByte::SosSpeed), 0, 0, 0]);

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
    fn write(&mut self, data: &mut [u8; 4]) {
        match self.port.as_mut() {
            Some(port) => port.write(data).expect("Could not write to port."),
            None => {
                let write_data = *data;

                self.write_buffers[0]
                    .get_mut()
                    .write(Packet::from(write_data));
                self.write_buffers[1]
                    .get_mut()
                    .write(Packet::from(write_data));
            }
        }
    }

    fn read(&mut self) -> Option<Packet> {
        match self.port.as_mut() {
            Some(com_port) => Some(com_port.read().expect("Failed to read from port.")),
            None => self.read_buffer.get_mut().read(),
        }
    }
}
