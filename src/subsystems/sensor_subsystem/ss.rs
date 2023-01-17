use crossbeam::channel::{self, Receiver, Sender};

use crate::{
    components::{
        adjacent_bytes::AdjacentBytes, buffer::BufferUser, colour::Colour, comm_port::ControlByte,
        constants::CAL_COLOURS, packet::Packet, state::SystemState,
    },
    gui::maze::MazeLineMap,
    subsystems::{comms_channel::CommsChannel, sensor_positions::SensorPosComputer},
};

#[derive(Debug)]
pub struct Ss {
    comms: CommsChannel,
    state: SystemState,
    positions_receiver: Receiver<[(f32, f32); 5]>,
}

impl Ss {
    pub fn new(
        comms: CommsChannel,
        mut position_calculator: SensorPosComputer,
        wheel_receiver: Receiver<(i16, i16)>,
    ) -> Self {
        let (pos_tx, pos_rx) = channel::bounded(1);

        std::thread::spawn(move || position_calculator.compute_pos(wheel_receiver, pos_tx));

        Self {
            state: SystemState::Idle,
            comms,
            positions_receiver: pos_rx,
        }
    }

    pub fn run(&mut self, maze: &MazeLineMap, positions_relay: &Sender<[(f32, f32); 5]>) {
        let mut end_of_maze = false;

        while !end_of_maze {
            match self.state {
                SystemState::Idle => {
                    /* IDLE */
                    let packet = self.read();
                    // if the control byte is correct, and a touch has been sensed
                    if packet.control_byte() == ControlByte::IdleButton && packet.dat1() == 1 {
                        self.write([112, 0, 0, 0]);
                        self.wait_for_packet(96.into());
                        self.state = SystemState::Calibrate;
                    }
                }
                SystemState::Calibrate => {
                    /* CALIBRATE */

                    self.wait_for_packet(97.into());

                    self.write(CAL_COLOURS);

                    if self.wait_for_packet(80.into()).dat1() == 1 {
                        self.state = SystemState::Maze;
                    }
                }
                SystemState::Maze => {
                    /* MAZE */

                    let mut colours = Vec::new();
                    let latest_positions = self.positions_receiver.recv().unwrap();
                    positions_relay.send(latest_positions);

                    // get the colours under each sensor
                    latest_positions.iter().for_each(|sensor_pos| {
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

                    self.wait_for_packet(164.into());

                    if end_of_maze {
                        self.write([179, 0, 0, 0]);
                    } else {
                        let mut word: u16 = 0;

                        for (index, colour) in colours.into_iter().enumerate() {
                            word |= (colour as u16) << 12 >> (index * 3);
                        }

                        let bytes: AdjacentBytes = word.into();

                        self.write([177, bytes.get_msb(), bytes.get_lsb(), 0]);
                        self.write([178, 0, 0, 0]);
                    }
                }
                SystemState::Sos => todo!(),
            }
        }
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
        let mut p: Packet = [0, 0, 0, 0].into();

        while p.control_byte() != control_byte {
            p = self.read();
        }

        p
    }
}
