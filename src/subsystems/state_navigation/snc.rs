//! # The SNC
//!
//! The state and navigation control (SNC) subsystem is responsible for controlling
//! the state of the system and navigating it through a maze.

use crate::components::{
    buffer::BufferUser, comm_port::ControlByte, packet::Packet, state::SystemState,
};

use crate::subsystems::comms_channel::CommsChannel;
use crate::subsystems::state_navigation::navcon::{NavCon, NavConState};

/// The struct that allows the system to emulate the SNC
#[derive(Debug)]
pub struct Snc {
    comms: CommsChannel,
    state: SystemState,
    navcon: NavCon,
}

impl Snc {
    /// creates a new instance of an `Snc`, required the read and write buffers
    /// during initialisation.
    ///
    /// `activate_port` will enable the COM Port (`ComPort`) if `true`
    ///
    /// need to add a way to set the COM port number and baud rate
    pub fn new(comms: CommsChannel) -> Self {
        Self {
            state: SystemState::Idle,
            navcon: NavCon::new(),
            comms,
        }
    }

    /// currently runs a single iteration of the SNC's state machine
    ///
    /// will most likely be changed to run asynchonously until maze completion
    pub fn run(&mut self) {
        let end_of_maze = false;

        let mut packets = [
            Packet::new(162, 0, 0, 0),
            Packet::new(163, 0, 0, 0),
            Packet::new(164, 0, 0, 0),
            Packet::new(177, 0, 0, 0),
            Packet::new(178, 0, 0, 0),
        ];

        while !end_of_maze {
            match self.state {
                SystemState::Idle => {
                    /* IDLE */
                    self.write([16, 1, 100, 0]); // write touch detected to port
                    self.state = SystemState::Calibrate; // go to calibrate state
                }
                SystemState::Calibrate => {
                    /* CALIBRATE */
                    let packet = self.read();
                    // ControlByte::CalibrateColours = 113
                    if packet.control_byte() == ControlByte::CalibrateColours {
                        self.write([80, 1, 0, 0]);
                        self.state = SystemState::Maze;
                    }
                }
                SystemState::Maze => {
                    /* MAZE */

                    self.write([145, 0, 0, 0]); // write no clap/snap sensed
                    self.write([146, 0, 0, 0]); // write no rouch

                    println!("navcon start");

                    // run NAVCON and write output:
                    self.navcon.compute_output(packets); // NAVCON

                    println!("navcon done");

                    // write navigation control data (Control byte = 147) based on navcon.compute_output()
                    match self.navcon.get_state() {
                        NavConState::Forward => self.write([147, 50, 50, 0]),
                        NavConState::Reverse => self.write([147, 50, 50, 1]),
                        NavConState::Stop => self.write([147, 0, 0, 0]),
                        NavConState::RotateLeft => {
                            let dat1 = ((self.navcon.output_rotation & 0xFF00) >> 8) as u8;
                            let dat0 = (self.navcon.output_rotation & 0x00FF) as u8;

                            self.write([147, dat1, dat0, 2]);
                        }
                        NavConState::RotateRight => {
                            let dat1 = ((self.navcon.output_rotation & 0xFF00) >> 8) as u8;
                            let dat0 = (self.navcon.output_rotation & 0x00FF) as u8;

                            self.write([147, dat1, dat0, 3]);
                        }
                    }
                    // get MDPS packets:
                    self.wait_for_packet(161.into()); // just discard the battery level packet

                    // now should be synchronised
                    for packet in &mut packets {
                        *packet = self.read();
                    }

                    // if end of maze, reset the packets and return to Idle
                    if packets[4].control_byte() == ControlByte::MazeEndOfMaze {
                        packets = [
                            Packet::new(162, 0, 0, 0),
                            Packet::new(163, 0, 0, 0),
                            Packet::new(164, 0, 0, 0),
                            Packet::new(177, 0, 0, 0),
                            Packet::new(178, 0, 0, 0),
                        ];

                        self.state = SystemState::Idle;
                    }

                    // --------------------------------------------------------------------------------------------
                }
                SystemState::Sos => {
                    /* SOS */
                    let packet = self.read();

                    if packet.control_byte() == ControlByte::SosSpeed {
                        self.write([208, 1, 0, 0]);
                        self.state = SystemState::Idle;
                    }
                }
            }
        }
    }
}

impl BufferUser for Snc {
    /// writes to the output buffer
    fn write(&mut self, data: [u8; 4]) {
        self.comms.send(data.into());
    }

    /// reads from the input buffer
    fn read(&mut self) -> Packet {
        self.comms.receive()
    }

    fn wait_for_packet(&mut self, control_byte: ControlByte) -> Packet {
        let mut p: Packet = [0, 0, 0, 0].into();

        while p.control_byte() != control_byte {
            p = self.read();
        }

        p
    }
}
