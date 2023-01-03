//! # The SNC
//!
//! The state and navigation control (SNC) subsystem is responsible for controlling
//! the state of the system and navigating it through a maze.

use crate::components::{
    buffer::{BufferUser, SharedBuffer},
    comm_port::ControlByte,
    packet::Packet,
    state::SystemState,
};

use crate::subsystems::state_navigation::navcon::{NavCon, NavConState};

use std::sync::Arc;

/// The struct that allows the system to emulate the SNC
#[derive(Debug)]
pub struct Snc {
    in_buffer: SharedBuffer,
    out_buffer: SharedBuffer,
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
    pub fn new(out_buffer: &SharedBuffer, in_buffer: &SharedBuffer) -> Self {
        Self {
            in_buffer: Arc::clone(in_buffer),
            out_buffer: Arc::clone(out_buffer),
            state: SystemState::Idle,
            navcon: NavCon::new(),
        }
    }

    /// currently runs a single iteration of the SNC's state machine
    ///
    /// will most likely be changed to run asynchonously until maze completion
    pub fn run(&mut self) {
        let mut end_of_maze = false;

        while !end_of_maze {
            match self.state {
                SystemState::Idle => {
                    /* IDLE */
                    self.write([16, 1, 100, 0]); // write touch detected to port
                    self.state = SystemState::Calibrate; // go to calibrate state
                }
                SystemState::Calibrate => {
                    /* CALIBRATE */
                    if let Some(packet) = self.read() {
                        // ControlByte::CalibrateColours = 113
                        if packet.control_byte() == ControlByte::CalibrateColours {
                            self.write([80, 1, 0, 0]);
                            self.state = SystemState::Maze;
                        }
                    }
                }
                SystemState::Maze => {
                    /* MAZE */

                    self.write([145, 0, 0, 0]); // write no clap/snap sensed
                    self.write([146, 0, 0, 0]); // write no rouch

                    let mut packets = [Packet::new(0, 0, 0, 0); 5];

                    // get MDPS packets:
                    self.wait_for_packet(161.into()); // just discard the battery level packet

                    packets[0] = self.wait_for_packet(162.into());
                    packets[1] = self.wait_for_packet(163.into());
                    packets[2] = self.wait_for_packet(164.into());

                    // get SS packets:
                    //
                    // first check if end of maze
                    let mut p_acket = None;

                    while p_acket.is_none() {
                        p_acket = self.read();
                    }

                    if p_acket.unwrap().control_byte() == 177.into() {
                        packets[3] = p_acket.unwrap();
                    } else {
                        end_of_maze = true;
                    }

                    packets[4] = self.wait_for_packet(178.into());
                    // --------------------------------------------------------------------------------------------

                    // run NAVCON and write output:
                    self.navcon.compute_output(packets); // NAVCON

                    // write navigation control data (Control byte = 147) based on navcon.compute_output()
                    match self.navcon.get_state() {
                        NavConState::Forward => self.write([147, 100, 100, 0]),
                        NavConState::Reverse => self.write([147, 100, 100, 1]),
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
                }
                SystemState::Sos => {
                    /* SOS */
                    if let Some(packet) = self.read() {
                        if packet.control_byte() == ControlByte::SosSpeed {
                            self.write([208, 1, 0, 0]);
                            self.state = SystemState::Idle;
                        }
                    }
                }
            }
        }
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

impl BufferUser for Snc {
    /// writes to the output buffer
    fn write(&mut self, data: [u8; 4]) {
        self.out_buffer.lock().unwrap().write(data.into());
    }

    /// reads from the input buffer
    fn read(&mut self) -> Option<Packet> {
        self.in_buffer.lock().unwrap().read()
    }
}
