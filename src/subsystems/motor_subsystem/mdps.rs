use crate::{
    asynchronous::{one_to_many_channel::OTMChannel, one_to_one_channel::OTOChannel},
    components::{
        adjacent_bytes::AdjacentBytes,
        buffer::BufferUser,
        comm_port::ControlByte,
        constants::{CAL_BATTERY_LEVEL, CAL_OPERATIONAL_VELOCITY, MAZE_BATTERY_LEVEL, SOS_SPEED},
        packet::Packet,
        state::SystemState,
    },
    subsystems::{motor_subsystem::wheel::Wheels, sensor_positions::Speeds},
};

/**
    # Motor-driver and Power Subsystem (MDPS) struct
    Provides a way to emulate the MDPS
**/
pub struct Mdps {
    comms: OTMChannel<Packet>,
    /// Wheels is a struct that contains data for speed, distance,
    /// and rotation. It facilitates some of the calculations
    /// required to keep track of this data in autonomous mode
    wheels: Wheels,
    /// The state that the full is in
    state: SystemState,
    /// The desired operating velocity during maze navigation
    operational_velocity: u8,
    speed_comms: OTOChannel<Speeds>,
}

impl Mdps {
    /// Returns a new MDPS
    ///
    /// If the test kit is running with one or more real subsystems, commport will be Some(..), otherwise it
    /// must be None
    pub fn new(comms: OTMChannel<Packet>, speed_comms: OTOChannel<Speeds>, wheels: Wheels) -> Self {
        Self {
            wheels,
            state: SystemState::Idle,
            operational_velocity: 0,
            comms,
            speed_comms,
        }
    }

    pub fn run(&mut self) {
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

                    // println!("MDPS going to cal");
                }
                SystemState::Calibrate => {
                    /* Calibration things */
                    self.wait_for_packet(112.into());

                    self.write(CAL_OPERATIONAL_VELOCITY);
                    self.write(CAL_BATTERY_LEVEL);

                    self.wait_for_packet(113.into());

                    while self.wait_for_packet(80.into()).dat1() != 1 {
                        /* wait for go to Maze state */
                        self.write(CAL_BATTERY_LEVEL);
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
                                        .set_left_wheel_speed(self.operational_velocity as i16);
                                    self.wheels
                                        .set_right_wheel_speed(-(self.operational_velocity as i16));

                                    rotating = true;
                                }
                                3 => {
                                    self.wheels
                                        .set_left_wheel_speed(-(self.operational_velocity as i16));
                                    self.wheels
                                        .set_right_wheel_speed(self.operational_velocity as i16);

                                    rotating = true;
                                }
                                _ => (),
                            };

                            self.wheels.update_distance();

                            if rotating {
                                let target_rotation: u16 =
                                    AdjacentBytes::make(packet.dat1(), packet.dat0()).into();

                                while self.wheels.get_rotation() < target_rotation {
                                    self.wheels.update_distance();

                                    let left_speed = self.wheels.get_left();
                                    let right_speed = self.wheels.get_right();

                                    self.speed_comms
                                        .send(Speeds::new(left_speed as f32, right_speed as f32));

                                    // match wheel_speeds
                                    //     .try_send((self.wheels.get_left(), self.wheels.get_right()))
                                    // {
                                    //     Ok(_) => println!(
                                    //         "successfully sent wheel speeds to calculator thread"
                                    //     ),
                                    //     Err(e) => {
                                    //         println!("INFO: mdps run thread could not send data to sensor positions calculator thread, {}", e);
                                    //         std::thread::sleep(Duration::from_millis(10));
                                    //     }
                                    // }
                                }
                            }

                            let left_speed = self.wheels.get_left();
                            let right_speed = self.wheels.get_right();

                            self.speed_comms
                                .send(Speeds::new(left_speed as f32, right_speed as f32));

                            // wheel_speeds.send((self.wheels.get_left(), self.wheels.get_right())).expect("FATAL: mdps run thread could not send data to sensor positions calculator thread");

                            // write battery level (no longer required as of 2022, so just send 0's)
                            self.write(MAZE_BATTERY_LEVEL);

                            // get final rotation
                            let curr_rotation = self.wheels.get_rotation();

                            let rotation_bytes = AdjacentBytes::from(curr_rotation);

                            self.write([
                                162,
                                rotation_bytes.lsb(),
                                rotation_bytes.msb(),
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

                            self.write([164, distance_bytes.msb(), distance_bytes.lsb(), 0]);
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
                    self.write(SOS_SPEED);

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
