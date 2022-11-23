use std::rc::Rc;

use crate::components::buffer::{Get, SharedBuffer};
use crate::components::comm_port::{ComPort, ControlByte};
use crate::components::packet::Packet;
use crate::components::state::SystemState;
use crate::subsystems::snc::navcon::{NavCon, NavConState};

#[derive(Debug)]
pub struct Snc {
    buffer: SharedBuffer,
    state: SystemState,
    port: Option<ComPort>,
    navcon: NavCon,
    auto: bool,
}

impl Snc {
    pub fn new(buffer: &SharedBuffer, activate_port: bool) -> Self {
        let comm_port;

        if activate_port {
            comm_port = Some(ComPort::new(String::from("69"), 19200));
        } else {
            comm_port = None;
        }

        Self {
            buffer: Rc::clone(buffer),
            state: SystemState::Idle,
            port: comm_port,
            navcon: NavCon::new(),
            auto: true,
        }
    }

    fn write(&mut self, data: &mut [u8; 4]) {
        if self.port.is_some() {
            self.port
                .as_mut()
                .unwrap()
                .write(data)
                .expect("Could not write to port in Maze state");
        } else {
            self.buffer.get_mut().write(Packet::from(*data));
        }
    }

    fn read(&mut self) -> Result<Packet, ()> {
        if self.port.is_some() {
            Ok(self
                .port
                .as_mut()
                .unwrap()
                .read()
                .expect("Failed to read from port in Calibrate"))
        } else {
            self.buffer.get_mut().read()
        }
    }

    pub fn run(&mut self) {
        match self.state {
            SystemState::Idle => {
                // write touch detected to port
                self.write(&mut [16, 1, 100, 0]);

                // go to calibrate state
                self.state = SystemState::Calibrate;

                println!("end idle");
            }
            SystemState::Calibrate => {
                if self.read().expect("No data to read").control_byte()
                    == ControlByte::CalibrateColours
                {
                    self.write(&mut [80, 1, 0, 0]);

                    self.write(&mut [145, 0, 0, 0]);
                    self.write(&mut [146, 0, 0, 0]);
                    self.write(&mut [147, 100, 100, 0]);

                    self.state = SystemState::Maze;
                }

                println!("end calibrate");
            }
            SystemState::Maze => {
                self.write(&mut [145, 0, 0, 0]);

                self.write(&mut [146, 0, 0, 0]);

                let expected_sequence = [
                    ControlByte::MazeBatteryLevel,
                    ControlByte::MazeRotation,
                    ControlByte::MazeSpeeds,
                    ControlByte::MazeRotation,
                    ControlByte::MazeColours,
                    ControlByte::MazeIncidence,
                ];

                let mut packets = Vec::from([Packet::new(0, 0, 0, 0); 6]);

                for (index, byte) in expected_sequence.iter().enumerate() {
                    while packets[index].control_byte() != *byte {
                        packets[index] =
                            Packet::from(self.read().expect("Failed to read input data in Maze"));
                    }
                }

                packets.remove(0);

                self.navcon
                    .compute_output(packets.as_slice().try_into().unwrap());

                match self.navcon.get_state() {
                    NavConState::Forward => self.write(&mut [147, 100, 100, 0]),
                    NavConState::Reverse => self.write(&mut [147, 100, 100, 1]),
                    NavConState::Stop => self.write(&mut [147, 0, 0, 0]),
                    NavConState::RotateLeft => {
                        let dat1 = ((self.navcon.output_rotation & 0xFF00) >> 8) as u8;
                        let dat0 = (self.navcon.output_rotation & 0x00FF) as u8;

                        self.write(&mut [147, dat1, dat0, 2]);
                    }
                    NavConState::RotateRight => {
                        let dat1 = ((self.navcon.output_rotation & 0xFF00) >> 8) as u8;
                        let dat0 = (self.navcon.output_rotation & 0x00FF) as u8;

                        self.write(&mut [147, dat1, dat0, 3]);
                    }
                }
            }
            SystemState::Sos => {
                while Packet::from(self.read().expect("Failed to read input data in Maze"))
                    .control_byte()
                    != ControlByte::SosSpeed
                { /* Do Noting */ }

                self.write(&mut [208, 1, 0, 0]);

                self.state = SystemState::Idle;
            }
        }
    }
}
