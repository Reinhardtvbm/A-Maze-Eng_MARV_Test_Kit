use std::f32::consts::PI;

use crate::{
    asynchronous::one_to_many_channel::OTMChannel,
    components::{
        adjacent_bytes::AdjacentBytes,
        buffer::BufferUser,
        colour::Colour,
        comm_port::ControlByte,
        constants::{B_ISD, CAL_CALIBRATED, CAL_COLOURS, MAZE_END_OF_MAZE},
        packet::Packet,
        state::SystemState,
    },
    gui::maze::MazeLineMap,
};

#[derive(Debug)]
pub struct Ss {
    comms: OTMChannel<Packet>,
    state: SystemState,
    curr_positions: [(f32, f32); 5],
    positions_channel: OTMChannel<[(f32, f32); 5]>,
    reference_distance: u16,
}

impl Ss {
    pub fn new(comms: OTMChannel<Packet>, positions_channel: OTMChannel<[(f32, f32); 5]>) -> Self {
        Self {
            state: SystemState::Idle,
            comms,
            curr_positions: [(0., 0.); 5],
            positions_channel,
            reference_distance: 0,
        }
    }

    pub fn run(&mut self, maze: &MazeLineMap) {
        let mut end_of_maze = false;

        while !end_of_maze {
            match self.state {
                SystemState::Idle => {
                    /* IDLE */
                    let packet = self.wait_for_packet(16.into());
                    // if the control byte is correct, and a touch has been sensed
                    if packet.dat1() == 1 {
                        self.state = SystemState::Calibrate;
                    }
                }
                SystemState::Calibrate => {
                    /* CALIBRATE */
                    self.write(CAL_CALIBRATED);
                    self.wait_for_packet(97.into());
                    self.write(CAL_COLOURS);

                    while self.wait_for_packet(80.into()).dat1() != 1 {
                        /* WAITING */
                        self.wait_for_packet(97.into());
                        self.write(CAL_COLOURS);
                    }

                    self.state = SystemState::Maze;
                }
                SystemState::Maze => {
                    /* MAZE */

                    let mut colours = Vec::new();

                    // NOTE: recv() blocks this thread until new data is received
                    if let Ok(new_positions) = self.positions_channel.try_receive() {
                        self.curr_positions = new_positions;
                    }

                    // get the colours under each sensor
                    self.curr_positions.iter().for_each(|sensor_pos| {
                        colours.push(
                            maze.get_colour_from_coord(sensor_pos.0, sensor_pos.1)
                                .expect("FATAL: colour in maze not found"),
                        )
                    });

                    if colours.iter().all(|colour| *colour == Colour::Red) {
                        end_of_maze = true;
                    }

                    if self.wait_for_packet(145.into()).dat1() == 1 {
                        self.state = SystemState::Sos;
                        break;
                    }

                    if self.wait_for_packet(146.into()).dat1() == 1 {
                        self.state = SystemState::Idle;
                        break;
                    }

                    let distance_packet = self.wait_for_packet(164.into());
                    let distance = u16::from(AdjacentBytes::make(
                        distance_packet.dat1(),
                        distance_packet.dat0(),
                    ));

                    if end_of_maze {
                        self.write(MAZE_END_OF_MAZE);
                    } else {
                        let mut word: u16 = 0;

                        println!("{:?}", colours);

                        if colours.contains(&Colour::Blue) {
                            println!("beewhoop");
                        }

                        let mut angle = 0;

                        if colours[0] != Colour::White || colours[4] != Colour::White {
                            self.reference_distance = distance;
                        } else if colours[1] != Colour::White
                            || colours[3] != Colour::White
                                && (colours[0] == Colour::White && colours[4] == Colour::White)
                        {
                            let mut travelled = distance as i16 - self.reference_distance as i16;

                            if travelled < 0 {
                                travelled = 0;
                            }

                            if colours.contains(&Colour::Red) {
                                println!("beep");
                            }

                            angle = ((travelled as f32 / B_ISD as f32).atan() * (180.0 / PI)) as u8;
                        }

                        for (index, colour) in colours.into_iter().enumerate() {
                            word |= (colour as u16) << 12 >> (index * 3);
                        }
                        let bytes: AdjacentBytes = word.into();

                        if angle >= 5 {
                            println!("weird");
                        }

                        self.write([177, bytes.msb(), bytes.lsb(), 0]);
                        self.write([178, angle, 0, 0]);
                    }
                }
                SystemState::Sos => todo!(),
            }
        }

        println!("SS run function ended");
    }
}

impl BufferUser for Ss {
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
