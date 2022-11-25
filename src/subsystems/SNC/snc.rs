use crate::components::buffer::{BufferUser, Get, SharedBuffer};
use crate::components::comm_port::{ComPort, ControlByte};
use crate::components::packet::Packet;
use crate::components::state::SystemState;
use crate::subsystems::snc::navcon::{NavCon, NavConState};
use std::rc::Rc;

#[derive(Debug)]
pub struct Snc {
    read_buffer: SharedBuffer,
    write_buffers: [SharedBuffer; 2],
    state: SystemState,
    port: Option<ComPort>,
    navcon: NavCon,
}

impl Snc {
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
            state: SystemState::Idle,
            port: comm_port,
            navcon: NavCon::new(),
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
            SystemState::Calibrate => match self.read() {
                Some(data) => {
                    if data.control_byte() == ControlByte::CalibrateColours {
                        self.write(&mut [80, 1, 0, 0]);
                        self.write(&mut [145, 0, 0, 0]);
                        self.write(&mut [146, 0, 0, 0]);
                        self.write(&mut [147, 100, 100, 0]);

                        self.state = SystemState::Maze;
                    }
                }
                None => println!("no new data in buffer (Calibrate)"),
            },
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

impl BufferUser for Snc {
    fn write(&mut self, data: &mut [u8; 4]) {
        if self.port.is_some() {
            self.port
                .as_mut()
                .unwrap()
                .write(&data)
                .expect("Could not write to port in Maze state");
        } else {
            let write_data = *data;

            self.write_buffers[0]
                .get_mut()
                .write(Packet::from(write_data));
            self.write_buffers[1]
                .get_mut()
                .write(Packet::from(write_data));
        }
    }

    fn read(&mut self) -> Option<Packet> {
        if self.port.is_some() {
            Some(
                self.port
                    .as_mut()
                    .unwrap()
                    .read()
                    .expect("Failed to read from port in Calibrate"),
            )
        } else {
            self.read_buffer.get_mut().read()
        }
    }
}
