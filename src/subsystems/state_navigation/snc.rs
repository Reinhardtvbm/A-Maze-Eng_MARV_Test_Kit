//! # The SNC
//!
//! The state and navigation control (SNC) subsystem is responsible for controlling
//! the state of the system and navigating it through a maze.

use crate::{
    components::{
        adjacent_bytes::AdjacentBytes,
        buffer::BufferUser,
        comm_port::ControlByte,
        constants::{
            CAL_BUTTON_TOUCHED, IDLE_BUTTON_TOUCHED, MAZE_BUTTON_NOT_TOUCHED, MAZE_CLAPSNAP_NONE,
            MAZE_NAVCON_FORWARD, MAZE_NAVCON_REVERSE, MAZE_NAVCON_STOP,
        },
        packet::Packet,
        state::SystemState,
    },
    subsystems::{
        channel::Channel,
        state_navigation::navcon::{NavCon, NavConState},
    },
};

/// The struct that allows the system to emulate the SNC
#[derive(Debug)]
pub struct Snc {
    comms: Channel<Packet>,
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
    pub fn new(comms: Channel<Packet>) -> Self {
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
        let mut end_of_maze = false;

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
                    self.write(IDLE_BUTTON_TOUCHED); // write touch detected to port
                    self.state = SystemState::Calibrate; // go to calibrate state
                }
                SystemState::Calibrate => {
                    /* CALIBRATE */
                    self.wait_for_packet(113.into());

                    self.write(CAL_BUTTON_TOUCHED);
                    self.state = SystemState::Maze;
                }
                SystemState::Maze => {
                    /* MAZE */

                    self.write(MAZE_CLAPSNAP_NONE); // write no clap/snap sensed
                    self.write(MAZE_BUTTON_NOT_TOUCHED); // write no rouch

                    // run NAVCON and write output:
                    self.navcon.compute_output(packets); // NAVCON

                    // write navigation control data (Control byte = 147) based on navcon.compute_output()
                    match self.navcon.get_state() {
                        NavConState::Forward => self.write(MAZE_NAVCON_FORWARD),
                        NavConState::Reverse => self.write(MAZE_NAVCON_REVERSE),
                        NavConState::Stop => self.write(MAZE_NAVCON_STOP),
                        NavConState::RotateLeft => {
                            let rotation = AdjacentBytes::from(self.navcon.output_rotation);
                            self.write([147, rotation.msb(), rotation.lsb(), 2]);
                            // panic!("{}", self.navcon.output_rotation);
                        }
                        NavConState::RotateRight => {
                            let rotation = AdjacentBytes::from(self.navcon.output_rotation);
                            self.write([147, rotation.msb(), rotation.lsb(), 3]);
                            //panic!("{}", self.navcon.output_rotation);
                        }
                    }
                    // get MDPS packets:
                    self.wait_for_packet(161.into()); // just discard the battery level packet

                    // now should be synchronised
                    for packet in &mut packets {
                        *packet = self.read();
                        if (*packet).control_byte() == ControlByte::MazeEndOfMaze {
                            end_of_maze = true;
                            break;
                        }
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

        println!("SNC run function ended");
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
        loop {
            let packet = self.read();

            if packet.control_byte() == control_byte {
                return packet;
            }
        }
    }
}
