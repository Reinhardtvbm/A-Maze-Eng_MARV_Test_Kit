use crossbeam::channel::Sender;

use crate::{
    components::{
        adjacent_bytes::AdjacentBytes, buffer::BufferUser, comm_port::ControlByte, packet::Packet,
        state::SystemState,
    },
    subsystems::{comms_channel::CommsChannel, motor_subsystem::wheel::Wheels},
};

/**
    # Motor-driver and Power Subsystem (MDPS) struct
    Provides a way to emulate the MDPS
**/
#[derive(Debug)]
pub struct Mdps {
    comms: CommsChannel,

    /// Wheels is a struct that contains data for speed, distance,
    /// and rotation. It facilitates some of the calculations
    /// required to keep track of this data in autonomous mode
    wheels: Wheels,
    /// The state that the full is in
    state: SystemState,
    /// The desired operating velocity during maze navigation
    operational_velocity: u8,
}

impl Mdps {
    /// Returns a new MDPS
    ///
    /// If the test kit is running with one or more real subsystems, commport will be Some(..), otherwise it
    /// must be None
    pub fn new(comms: CommsChannel, wheels: Wheels) -> Self {
        Self {
            wheels,
            state: SystemState::Idle,
            operational_velocity: 0,
            comms,
        }
    }

    pub fn run(&mut self, wheel_speeds: Sender<(i16, i16)>) {
        let mut end_of_maze = false;

        while !end_of_maze {
            match self.state {
                SystemState::Idle => {
                    /* Idle things */
                    /* IDLE */
                    let packet = self.wait_for_packet(16.into());
                    // if the control byte is correct, and a touch has been sensed
                    if packet.dat1() == 1 {
                        self.operational_velocity = packet.dat0();
                        self.state = SystemState::Calibrate;
                    }
                }
                SystemState::Calibrate => {
                    /* Calibration things */
                    self.wait_for_packet(112.into());

                    self.write([
                        u8::from(ControlByte::CalibrateOperationalVelocity),
                        self.operational_velocity,
                        self.operational_velocity,
                        0,
                    ]);

                    self.write([96, 0, 0, 0]);
                    self.write([97, 0, 0, 0]);

                    self.wait_for_packet(113.into());

                    while self.wait_for_packet(80.into()).dat1() != 1 {
                        /* wait for go to Maze state */
                        self.write([97, 0, 0, 0]);
                        self.wait_for_packet(113.into());
                    }

                    self.state = SystemState::Maze;
                }
                SystemState::Maze => {
                    /* Maze things */
                    let packet = self.read();

                    match packet.control_byte() {
                        ControlByte::MazeClapSnap => {
                            if packet.dat1() == 1 {
                                self.state = SystemState::Sos;
                            }
                        }
                        ControlByte::MazeButton => {
                            if packet.dat1() == 1 {
                                self.state = SystemState::Idle;
                            }
                        }
                        ControlByte::MazeNavInstructions => {
                            // println!("{:?}", packet);
                            let mut rotating = false;

                            let (left, right) = (packet.dat1(), packet.dat0());

                            match packet.dec() {
                                0 => {
                                    self.wheels.set_left_wheel_speed(left as i16);
                                    self.wheels.set_right_wheel_speed(right as i16);
                                }
                                1 => {
                                    self.wheels.set_left_wheel_speed(-(left as i16));
                                    self.wheels.set_right_wheel_speed(-(right as i16));
                                }
                                2 => {
                                    self.wheels
                                        .set_left_wheel_speed(-(self.operational_velocity as i16));
                                    self.wheels
                                        .set_right_wheel_speed(self.operational_velocity as i16);

                                    rotating = true;
                                }
                                3 => {
                                    self.wheels
                                        .set_left_wheel_speed(self.operational_velocity as i16);
                                    self.wheels
                                        .set_right_wheel_speed(-(self.operational_velocity as i16));

                                    rotating = true;
                                }
                                _ => (),
                            };

                            self.wheels.update_distance();

                            if rotating {
                                let target_rotation =
                                    AdjacentBytes::make(packet.dat1(), packet.dat0()).into();

                                while self.wheels.get_rotation() < target_rotation {
                                    self.wheels.update_distance();

                                    wheel_speeds
                                        .send((self.wheels.get_left(), self.wheels.get_right())).expect("FATAL: mdps run thread could not send data to sensor positions calculator thread");

                                    //println!("MDPS thread sending wheels");
                                }
                            }

                            wheel_speeds.send((self.wheels.get_left(), self.wheels.get_right())).expect("FATAL: mdps run thread could not send data to sensor positions calculator thread");

                            // write battery level (no longer required as of 2022, so just send 0's)
                            self.write([161, 0, 0, 0]);

                            // get final rotation
                            let curr_rotation = self.wheels.get_rotation();

                            let rotation_bytes = AdjacentBytes::from(curr_rotation);

                            self.write([
                                162,
                                rotation_bytes.get_lsb(),
                                rotation_bytes.get_msb(),
                                match self.wheels.left_rotation() {
                                    true => 2,
                                    false => 3,
                                },
                            ]);

                            // write speed
                            self.write([
                                163,
                                self.wheels.get_left_wheel_speed(),
                                self.wheels.get_right_wheel_speed(),
                                match self.wheels.going_forward() {
                                    true => 0,
                                    false => 1,
                                },
                            ]);

                            // write distance
                            let distance = self.wheels.get_distance();

                            let distance_bytes = AdjacentBytes::from(distance);

                            self.write([
                                164,
                                distance_bytes.get_msb(),
                                distance_bytes.get_lsb(),
                                0,
                            ]);
                        }
                        ControlByte::MazeEndOfMaze => end_of_maze = true,
                        _ => (),
                    }
                }
                SystemState::Sos => {
                    /* SOS things */

                    // stop the MARV
                    self.wheels.set_left_wheel_speed(0);
                    self.wheels.set_right_wheel_speed(0);

                    // send that we have stopped
                    self.write([u8::from(ControlByte::SosSpeed), 0, 0, 0]);

                    // wait for clap/snap
                    while self.wait_for_packet(208.into()).dat1() != 1 {
                        // (do nothing / wait for end of SOS)
                    }

                    self.state = SystemState::Maze;
                }
            }
        }

        println!("MDPS run function ended");
    }
}

impl BufferUser for Mdps {
    /// writes to the output buffer
    fn write(&mut self, data: [u8; 4]) {
        //println!("MDPS sending...");
        self.comms.send(data.into());
    }

    /// reads from the input buffer
    fn read(&mut self) -> Packet {
        let p = self.comms.receive();
        //println!("MDPS got {:?}", p);
        p
    }

    fn wait_for_packet(&mut self, control_byte: ControlByte) -> Packet {
        //println!("MDPS waiting for packet ({:?})", control_byte);
        let mut p: Packet = [0, 0, 0, 0].into();

        while p.control_byte() != control_byte {
            p = self.read();
        }

        p
    }
}
