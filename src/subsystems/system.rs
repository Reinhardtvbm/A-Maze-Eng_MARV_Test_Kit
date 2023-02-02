//! The bread and butter of the program:
//!     will emulate the maze robot

use std::sync::{Arc, Mutex};

use crate::asynchronous::one_to_many_channel::{Bound, OTMChannel};
use crate::asynchronous::one_to_one_channel::OTOChannel;
use crate::components::buffer::Buffer;
use crate::components::packet::Packet;
use crate::gui::maze::MazeLineMap;

use crate::subsystems::{
    motor_subsystem::mdps::Mdps, sensor_subsystem::ss::Ss, state_navigation::snc::Snc,
};

use super::motor_subsystem::wheel::Wheels;
use super::sensor_positions::SensorPosComputer;
use super::serial_relay::SerialRelay;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Emulate,
    Physical,
}

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
    start_angle: f32,
    // positions data going to the GUI thread
    to_gui: &Arc<Mutex<Buffer<[(f32, f32); 5]>>>,
) {
    let wheels = Wheels::new(10.0);
    let thread;

    // ENDPOINT variables:

    // endpoints for speeds data going to and from the mdps and sensor positions computer threads
    let to_mdps_speeds = Arc::new(Mutex::new(Buffer::new()));
    let to_pos_computer_speeds = Arc::new(Mutex::new(Buffer::new()));

    // endpoints for positions data going to and from the ss and sensor positions computer threads
    let to_ss_positions = Arc::new(Mutex::new(Buffer::new()));
    let to_pos_computer_positions = Arc::new(Mutex::new(Buffer::new()));

    // endpoints for packets between subsystem threads
    let to_snc = Arc::new(Mutex::new(Buffer::new()));
    let to_ss = Arc::new(Mutex::new(Buffer::new()));
    let to_mdps = Arc::new(Mutex::new(Buffer::new()));

    // ==================================================================================================================

    // CHANNEL variables:

    // packet channels (comms between 3 threads):
    let snc_channel: OTMChannel<Packet> =
        OTMChannel::with_endpoints("SNC", &to_snc, vec![&to_ss, &to_mdps], Bound::Inifinity);
    let ss_channel: OTMChannel<Packet> =
        OTMChannel::with_endpoints("SS", &to_ss, vec![&to_snc, &to_mdps], Bound::Inifinity);
    let mdps_channel: OTMChannel<Packet> =
        OTMChannel::with_endpoints("MDPS", &to_mdps, vec![&to_snc, &to_ss], Bound::Inifinity);

    // speeds channels (comms between 2 threads):
    let sensor_pos_comms_speeds = OTOChannel::new(
        "Sensor Positions Channel (Speeds)",
        &to_pos_computer_speeds,
        &to_mdps_speeds,
    );

    let mdps_comms_speeds =
        OTOChannel::new("MDPS (Speeds)", &to_mdps_speeds, &to_pos_computer_speeds);

    // positions channels (comms between 3 threads):
    // NOTE: only two channels being created since GUI will read directly from its `Arc` in the outer scope
    let sensor_pos_comms_positions = OTMChannel::with_endpoints(
        "Sensor Positions Channel (Positions)",
        &to_pos_computer_positions,
        vec![to_gui, &to_ss_positions],
        Bound::Finite(1),
    );

    let ss_comms_positions = OTMChannel::with_endpoints(
        "SS (Positions)",
        &to_ss_positions,
        vec![to_gui, &to_pos_computer_positions],
        Bound::Finite(1),
    );

    // ==================================================================================================================

    let mut sensor_position_computer = SensorPosComputer::new(
        start_pos.0,
        start_pos.1,
        sensor_pos_comms_speeds,
        sensor_pos_comms_positions,
        start_angle,
    );

    std::thread::spawn(move || sensor_position_computer.compute_pos());

    // ==================================================================================================================

    // run their emulations if required, or setup a serial port relay if not
    match snc_mode {
        Mode::Emulate => {
            let mut snc = Snc::new(snc_channel);
            thread = std::thread::spawn(move || snc.run());
        }
        Mode::Physical => {
            let mut relay = SerialRelay::new(snc_channel, "10");
            thread = std::thread::spawn(move || relay.run());
        }
    }

    match ss_mode {
        Mode::Emulate => {
            let mut ss = Ss::new(ss_channel, ss_comms_positions);
            std::thread::spawn(move || ss.run(&maze));
        }
        Mode::Physical => {
            let mut relay = SerialRelay::new(ss_channel, "10");
            std::thread::spawn(move || relay.run());
        }
    }

    match mdps_mode {
        Mode::Emulate => {
            let mut mdps = Mdps::new(mdps_channel, mdps_comms_speeds, wheels);
            std::thread::spawn(move || mdps.run());
        }
        Mode::Physical => {
            let mut relay = SerialRelay::new(mdps_channel, "10");
            std::thread::spawn(move || relay.run());
        }
    }

    thread.join().expect("could not join SNC thread");
    println!("system function ended");
}
