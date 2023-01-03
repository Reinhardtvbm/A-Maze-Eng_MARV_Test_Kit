//! The bread and butter of the program:
//!     will emulate the maze robot

use std::sync::{Arc, Mutex};

use crate::components::buffer::{Buffer, SharedBuffer};

use crate::components::comm_port::ControlByte;
use crate::subsystems::{
    motor_subsystem::mdps::Mdps, sensor_subsystem::ss::Ss, state_navigation::snc::Snc,
};

use super::serial_relay::SerialRelay;

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

impl System {}

pub fn run_system(
    snc_mode: Mode,
    mdps_mode: Mode,
    ss_mode: Mode,
    snc_com: String,
    ss_com: String,
    mdps_com: String,
) {
    // declare each subsystem's I/O buffers
    let snc_input_buffer = Arc::new(Mutex::new(Buffer::new()));
    let ss_input_buffer = Arc::new(Mutex::new(Buffer::new()));
    let mdps_input_buffer = Arc::new(Mutex::new(Buffer::new()));

    let snc_output_buffer = Arc::new(Mutex::new(Buffer::new()));
    let ss_output_buffer = Arc::new(Mutex::new(Buffer::new()));
    let mdps_output_buffer = Arc::new(Mutex::new(Buffer::new()));

    // run their emulations if required, or setup a serial port relay if not
    match snc_mode {
        Mode::Emulate => {
            let mut snc = Snc::new(&snc_output_buffer, &snc_input_buffer);
            std::thread::spawn(move || snc.run());
        }
        Mode::Physical => {
            let mut relay = SerialRelay::new(&snc_output_buffer, &snc_input_buffer, snc_com);
            std::thread::spawn(move || relay.run());
        }
    }

    match ss_mode {
        Mode::Emulate => {
            let mut ss = Ss::new(&ss_output_buffer, &ss_input_buffer, [(0., 0.); 5]);
            std::thread::spawn(move || ss.run());
        }
        Mode::Physical => {
            let mut relay = SerialRelay::new(&ss_output_buffer, &ss_input_buffer, ss_com);
            std::thread::spawn(move || relay.run());
        }
    }

    match mdps_mode {
        Mode::Emulate => {
            let mut mdps = Mdps::new(&mdps_output_buffer, &mdps_input_buffer);
            std::thread::spawn(move || mdps.run());
        }
        Mode::Physical => {
            let mut relay = SerialRelay::new(&mdps_output_buffer, &mdps_input_buffer, mdps_com);
            std::thread::spawn(move || relay.run());
        }
    }

    let mut maze_end = false;

    while !maze_end {
        let snc_data = snc_output_buffer.lock().unwrap().read();
        let ss_data = ss_output_buffer.lock().unwrap().read();
        let mdps_data = mdps_output_buffer.lock().unwrap().read();

        if let Some(packet) = snc_data {
            ss_input_buffer.lock().unwrap().write(packet);
            mdps_input_buffer.lock().unwrap().write(packet);
        }

        if let Some(packet) = ss_data {
            snc_input_buffer.lock().unwrap().write(packet);
            mdps_input_buffer.lock().unwrap().write(packet);

            if packet.control_byte() == ControlByte::MazeEndOfMaze {
                maze_end = true;
            }
        }

        if let Some(packet) = mdps_data {
            ss_input_buffer.lock().unwrap().write(packet);
            snc_input_buffer.lock().unwrap().write(packet);
        }
    }
}
