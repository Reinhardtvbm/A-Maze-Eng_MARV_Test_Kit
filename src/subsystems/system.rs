//! The bread and butter of the program:
//!     will emulate the maze robot

use std::sync::{Arc, Mutex};

use crate::components::buffer::{Buffer, SharedBuffer};

use crate::components::comm_port::ControlByte;
use crate::subsystems::{
    motor_subsystem::mdps::Mdps, sensor_subsystem::ss::Ss, state_navigation::snc::Snc,
};

#[derive(Debug, PartialEq)]
pub enum Mode {
    Emulate,
    Physical,
}

#[derive(Debug)]
pub struct System {
    pub snc: Option<Snc>,
    pub ss: Option<Ss>,
    pub mdps: Option<Mdps>,
    pub buffer: SharedBuffer,
}

impl System {
    pub fn new(snc_mode: Mode, mdps_mode: Mode, ss_mode: Mode) -> Self {
        let snc_buffer = Arc::new(Mutex::new(Buffer::new()));
        let ss_buffer = Arc::new(Mutex::new(Buffer::new()));
        let mdps_buffer = Arc::new(Mutex::new(Buffer::new()));

        let snc = match snc_mode {
            Mode::Emulate => Some(Snc::new((&ss_buffer, &mdps_buffer), &snc_buffer, false)),
            Mode::Physical => None,
        };

        let mdps = match mdps_mode {
            Mode::Emulate => Some(Mdps::new((&ss_buffer, &snc_buffer), &mdps_buffer, false)),
            Mode::Physical => None,
        };

        let ss = match ss_mode {
            Mode::Emulate => Some(Ss::new(
                (&snc_buffer, &mdps_buffer),
                &snc_buffer,
                false,
                [(0.0, 0.0); 5],
            )),
            Mode::Physical => None,
        };

        Self {
            snc,
            ss,
            mdps,
            buffer: Arc::clone(&snc_buffer),
        }
    }

    pub fn run(self) {
        if let Some(mut snc) = self.snc {
            std::thread::spawn(move || snc.run());
        }

        if let Some(mut mdps) = self.mdps {
            std::thread::spawn(move || mdps.run());
        }

        let mut continue_maze = true;

        while continue_maze {
            if let Some(packet) = self.buffer.lock().unwrap().peek() {
                if packet.control_byte() == ControlByte::MazeEndOfMaze {
                    continue_maze = false;
                }
            }
        }
    }
}
