//! # The SNC
//!
//! The state and navigation control (SNC) subsystem is responsible for controlling
//! the state of the system and navigating it through a maze.

use crate::components::{
    buffer::{BufferUser, Get, SharedBuffer},
    comm_port::{ComPort, ControlByte},
    packet::Packet,
    state::SystemState,
};

use crate::subsystems::state_navigation::navcon::{NavCon, NavConState};

use std::{rc::Rc, sync::Arc};

/// The struct that allows the system to emulate the SNC
#[derive(Debug)]
pub struct Snc {
    read_buffer: SharedBuffer,
    write_buffers: [SharedBuffer; 2],
    state: SystemState,
    port: Option<ComPort>,
    navcon: NavCon,
}

impl Snc {
    /// creates a new instance of an `Snc`, required the read and write buffers
    /// during initialisation.
    ///
    /// `activate_port` will enable the COM Port (`ComPort`) if `true`
    ///
    /// need to add a way to set the COM port number and baud rate
    pub fn new(
        w_buffers: (&SharedBuffer, &SharedBuffer),
        r_buffer: &SharedBuffer,
        activate_port: bool,
    ) -> Self {
        let comm_port = match activate_port {
            true => Some(ComPort::new(String::from("69"), 19200)),
            false => None,
        };

        Self {
            read_buffer: Arc::clone(r_buffer),
            write_buffers: [Arc::clone(w_buffers.0), Arc::clone(w_buffers.1)],
            state: SystemState::Idle,
            port: comm_port,
            navcon: NavCon::new(),
        }
    }

    /// currently runs a single iteration of the SNC's state machine
    ///
    /// will most likely be changed to run asynchonously until maze completion
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
                    // ControlByte::CalibrateColours = 113
                    if data.control_byte() == ControlByte::CalibrateColours {
                        self.write(&mut [80, 1, 0, 0]);
                        self.state = SystemState::Maze;
                    }
                }
                None => println!("no new data in buffer (Calibrate)"),
            },
            SystemState::Maze => {
                // write no clap/snap sensed
                self.write(&mut [145, 0, 0, 0]);
                // write no rouch
                self.write(&mut [146, 0, 0, 0]);

                // NOTE:
                //  this is to ensure the packets are received in the correct order
                //  may need refactoring in the future
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
                        packets[index] = self.read().expect("Failed to read input data in Maze");
                    }
                }

                packets.remove(0);

                self.navcon
                    .compute_output(packets.as_slice().try_into().unwrap());

                // write navigation control data (Control byte = 147) based on navcon.compute_output()
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
                if let Some(packet) = self.read() {
                    if packet.control_byte() == ControlByte::SosSpeed {
                        self.write(&mut [208, 1, 0, 0]);
                        self.state = SystemState::Idle;
                    }
                }
            }
        }
    }
}

impl BufferUser for Snc {
    /// writes `data` to both read buffers and to the COM port if its in use.
    /// We always write to the buffers so that the gui can look at the packets.
    fn write(&mut self, data: &mut [u8; 4]) {
        if let Some(com_port) = &mut self.port {
            com_port.write(data).expect("SNC could not write to port.");
        }

        let write_data = *data;

        self.write_buffers
            .iter()
            .for_each(|buffer| buffer.lock().unwrap().write(write_data.into()));
    }

    /// reads from the read buffer
    /// will return `None` if there are no poackets in the buffer, and
    /// `Some(Packet)` if there is
    fn read(&mut self) -> Option<Packet> {
        let port_data;
        let buffer_data = self.read_buffer.lock().unwrap().read();

        if let Some(com_port) = &mut self.port {
            port_data = com_port.read().expect("Failed to read from port.");

            // here we do a sanity check, just to make sure
            if buffer_data.unwrap() != port_data {
                panic!("FATAL: serial port data does not match buffer data");
            }
        }

        buffer_data
    }
}
