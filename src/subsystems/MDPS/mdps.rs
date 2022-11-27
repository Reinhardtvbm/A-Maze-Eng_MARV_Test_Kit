use std::rc::Rc;

use crate::components::{
    buffer::{BufferUser, Get, SharedBuffer},
    comm_port::{ComPort, ControlByte},
    packet::{self, Packet},
    state::SystemState,
};

#[derive(Debug)]
pub struct Mdps {
    read_buffer: SharedBuffer,
    write_buffers: [SharedBuffer; 2],
    port: Option<ComPort>,
    state: SystemState,
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
            state: SystemState::Idle,
            operational_velocity: 0,
        }
    }

    pub fn run(&mut self) {
        match self.state {
            SystemState::Idle => match self.read() {
                /* Idle things */
                Some(packet) => {
                    if packet.control_byte() == ControlByte::IdleButton && packet.dat1() == 1 {
                        self.operational_velocity = packet.dat0();
                        self.state = SystemState::Calibrate;
                    }
                }
                None => (),
            },
            SystemState::Calibrate => {
                /* Calibration things */
                match self.read() {
                    Some(packet) => match packet.control_byte() {
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
                    },
                    None => (),
                }
            }
            SystemState::Maze => {
                /* Maze things */
                match self.read() {
                    Some(packet) => match packet.control_byte() {
                        ControlByte::MazeClapSnap => todo!(),
                        ControlByte::MazeButton => todo!(),
                        ControlByte::MazeNavInstructions => match packet.dec() {
                            0 => {}
                            1 => {}
                            2 => {}
                            3 => {}
                            _ => (),
                        },
                        ControlByte::MazeEndOfMaze => self.state = SystemState::Idle,
                        _ => (),
                    },
                    None => (),
                }
            }
            SystemState::Sos => { /* SOS things */ }
        }
    }
}

impl BufferUser for Mdps {
    fn write(&mut self, data: &mut [u8; 4]) {
        match self.port.as_mut() {
            Some(port) => port.write(&data).expect("Could not write to port."),
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
