use std::sync::Arc;

use crate::components::buffer::Buffer;
use crate::components::comm_port::{self, ComPort, ControlByte};
use crate::components::packet::Packet;
use crate::components::state::SystemState;
use crate::subsystems::snc::navcon::{NavCon, NavConState};

#[derive(Debug)]
pub struct Snc {
    buffer: Arc<Buffer>,
    state: SystemState,
    port: Option<ComPort>,
    navcon: NavCon,
    auto: bool,
}

impl Snc {
    pub fn new(buffer: &Arc<Buffer>, activate_port: bool) -> Self {
        let comm_port;

        if activate_port {
            comm_port = Some(ComPort::new(String::from("69"), 19200));
        } else {
            comm_port = None;
        }

        Self {
            buffer: Arc::clone(buffer),
            state: SystemState::Idle,
            port: comm_port,
            navcon: NavCon::new(),
            auto: true,
        }
    }

    fn write(&self, data: &[u8; 4]) {
        if self.port.is_some() {
            self.port
                .unwrap()
                .write(data)
                .expect("Could not write to port in Maze state");
        }
    }

    pub fn run(&mut self) {
        match self.state {
            SystemState::Idle => {
                // write touch detected to port
                self.write(&[16, 1, 100, 0]);

                // go to calibrate state
                self.state = SystemState::Calibrate;
            }
            SystemState::Calibrate => {
                if self
                    .port
                    .read()
                    .expect("Failed to read from port in Calibrate")
                    .control_byte()
                    == ControlByte::CalibrateColours
                {
                    self.port
                        .write(&[80, 1, 0, 0])
                        .expect("Failed to write touch in Calibrate");

                    self.port
                        .write(&[145, 0, 0, 0])
                        .expect("Failed to write clap/snap in Calibrate");

                    self.port
                        .write(&[146, 0, 0, 0])
                        .expect("Failed to write touch in Calibrate");

                    self.port
                        .write(&[147, 100, 100, 0])
                        .expect("Failed to write forward in Calibrate");

                    self.state = SystemState::Maze;
                }
            }
            SystemState::Maze => {
                self.port
                    .write(&[145, 0, 0, 0])
                    .expect("Failed to write clap/snap in Maze");

                self.port
                    .write(&[146, 0, 0, 0])
                    .expect("Failed to write touch in Maze");

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
                        packets[index] = Packet::from(
                            self.port.read().expect("Failed to read input data in Maze"),
                        );
                    }
                }

                packets.remove(0);

                self.navcon
                    .compute_output(packets.as_slice().try_into().unwrap());

                match self.navcon.get_state() {
                    NavConState::Forward => self
                        .port
                        .write(&[147, 100, 100, 0])
                        .expect("Could not write NAVCON data in Maze"),
                    NavConState::Reverse => self
                        .port
                        .write(&[147, 100, 100, 1])
                        .expect("Could not write NAVCON data in Maze"),
                    NavConState::Stop => self
                        .port
                        .write(&[147, 0, 0, 0])
                        .expect("Could not write NAVCON data in Maze"),
                    NavConState::RotateLeft => {
                        let dat1 = ((self.navcon.output_rotation & 0xFF00) >> 8) as u8;
                        let dat0 = (self.navcon.output_rotation & 0x00FF) as u8;

                        self.port
                            .write(&[147, dat1, dat0, 2])
                            .expect("Could not write NAVCON data in Maze");
                    }
                    NavConState::RotateRight => {
                        let dat1 = ((self.navcon.output_rotation & 0xFF00) >> 8) as u8;
                        let dat0 = (self.navcon.output_rotation & 0x00FF) as u8;

                        self.port
                            .write(&[147, dat1, dat0, 3])
                            .expect("Could not write NAVCON data in Maze");
                    }
                }
            }
            SystemState::Sos => {
                while Packet::from(self.port.read().expect("Failed to read input data in Maze"))
                    .control_byte()
                    != ControlByte::SosSpeed
                { /* Do Noting */ }

                self.port
                    .write(&[208, 1, 0, 0])
                    .expect("Could not write clap/snap in SOS");

                self.state = SystemState::Idle;
            }
        }
    }
}
