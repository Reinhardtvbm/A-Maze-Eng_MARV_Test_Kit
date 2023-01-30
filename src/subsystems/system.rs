//! The bread and butter of the program:
//!     will emulate the maze robot

use std::sync::{Arc, Mutex};

use crate::components::buffer::Buffer;
use crate::components::packet::Packet;
use crate::gui::maze::MazeLineMap;
use crate::subsystems::channel::Channel;
use crate::subsystems::{
    motor_subsystem::mdps::Mdps, sensor_subsystem::ss::Ss, state_navigation::snc::Snc,
};
use crossbeam::channel::{self, Sender};

use super::comms_channel::CommsChannel;
use super::motor_subsystem::wheel::Wheels;
use super::sensor_positions::SensorPosComputer;
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
}

impl System {}

pub fn run_system(
    snc_mode: Mode,
    mdps_mode: Mode,
    ss_mode: Mode,
    _snc_com: &str,
    _ss_com: &str,
    _mdps_com: &str,
    maze: MazeLineMap,
    start_pos: (f32, f32),
    _start_angle: f32,
    gui_sender: Sender<[(f32, f32); 5]>,
) {
    // declare each subsystem's I/O buffers
    let (mdps_tx1, ss_rx1) = channel::unbounded();
    let (mdps_tx2, snc_rx1) = channel::unbounded();

    let (ss_tx1, mdps_rx1) = channel::unbounded();
    let (ss_tx2, snc_rx2) = channel::unbounded();

    let (snc_tx1, ss_rx2) = channel::unbounded();
    let (snc_tx2, mdps_rx2) = channel::unbounded();

    // let (sensor_positions_tx, sensor_positions_rx) = channel::bounded(1);
    let (wheel_tx, wheel_rx) = channel::bounded(1);

    let sensor_position_computer = SensorPosComputer::new(start_pos.0, start_pos.1);

    let snc_comms = CommsChannel::new((snc_tx1, snc_tx2), (snc_rx1, snc_rx2), "SNC");
    let ss_comms = CommsChannel::new((ss_tx1, ss_tx2), (ss_rx1, ss_rx2), "SS");
    let mdps_comms = CommsChannel::new((mdps_tx1, mdps_tx2), (mdps_rx1, mdps_rx2), "MDPS");

    let wheels = Wheels::new(10.0);
    let thread;

    let to_snc = Arc::new(Mutex::new(Buffer::new()));
    let to_ss = Arc::new(Mutex::new(Buffer::new()));
    let to_mdps = Arc::new(Mutex::new(Buffer::new()));

    let snc_channel: Channel<Packet> =
        Channel::with_endpoints("SNC", &to_snc, vec![&to_ss, &to_mdps]);
    let ss_channel: Channel<Packet> =
        Channel::with_endpoints("SS", &to_ss, vec![&to_snc, &to_mdps]);
    let mdps_channel: Channel<Packet> =
        Channel::with_endpoints("MDPS", &to_mdps, vec![&to_snc, &to_ss]);

    // run their emulations if required, or setup a serial port relay if not
    match snc_mode {
        Mode::Emulate => {
            let mut snc = Snc::new(snc_channel);
            thread = std::thread::spawn(move || snc.run());
        }
        Mode::Physical => {
            let mut relay = SerialRelay::new(snc_comms, String::from("10"));
            thread = std::thread::spawn(move || relay.run());
        }
    }

    match ss_mode {
        Mode::Emulate => {
            let mut ss = Ss::new(ss_channel, sensor_position_computer, wheel_rx, gui_sender);
            std::thread::spawn(move || ss.run(&maze));
        }
        Mode::Physical => {
            let mut relay = SerialRelay::new(ss_comms, String::from("10"));
            std::thread::spawn(move || relay.run());
        }
    }

    match mdps_mode {
        Mode::Emulate => {
            let mut mdps = Mdps::new(mdps_channel, wheels);
            std::thread::spawn(move || mdps.run(wheel_tx));
        }
        Mode::Physical => {
            let mut relay = SerialRelay::new(mdps_comms, String::from("10"));
            std::thread::spawn(move || relay.run());
        }
    }

    thread.join().expect("could not join SNC thread");
    println!("system function ended");
}
