use std::sync::Arc;

use crate::{
    components::{
        buffer::{BufferUser, SharedBuffer},
        colour::Colours,
        comm_port::ControlByte,
        packet::Packet,
        state::SystemState,
    },
    gui::maze::MazeLineMap,
};

#[derive(Debug)]
pub struct Ss {
    /// the x-y coordinates of each sensor, these will be
    /// updated using the distance and rotation measurements
    /// from the MDPS
    sensor_positions: [(f32, f32); 5],
    /// the colour that each sensor senses, will get via the
    /// `get_colour_from_coord` method in the `maze` module
    sensor_colours: Colours,
    /// A shared buffer of type Rc<RefCell<_>>
    /// which is written to by the other two subsystems
    in_buffer: SharedBuffer,
    /// The shared buffers of the other two subsystems
    /// for the MDPS to send its data to
    out_buffer: SharedBuffer,
    state: SystemState,
}

impl Ss {
    pub fn new(
        out_buffer: &SharedBuffer,
        in_buffer: &SharedBuffer,
        init_sensor_pos: [(f32, f32); 5],
    ) -> Self {
        Self {
            sensor_colours: Colours::new(),
            sensor_positions: init_sensor_pos,
            in_buffer: Arc::clone(in_buffer),
            out_buffer: Arc::clone(out_buffer),
            state: SystemState::Idle,
        }
    }

    pub fn run(&mut self, maze: &MazeLineMap) {
        let mut end_of_maze = false;

        while !end_of_maze {
            match self.state {
                SystemState::Idle => {
                    /* IDLE */

                    if let Some(packet) = self.read() {
                        // if the control byte is correct, and a touch has been sensed
                        if packet.control_byte() == ControlByte::IdleButton && packet.dat1() == 1 {
                            self.state = SystemState::Calibrate;
                            self.write([112, 0, 0, 0]);
                            self.wait_for_packet(96.into());
                        }
                    }
                }
                SystemState::Calibrate => {
                    /* CALIBRATE */

                    self.wait_for_packet(97.into());

                    self.write([113, 0, 0, 0]);

                    if self.wait_for_packet(80.into()).dat1() == 1 {
                        self.state = SystemState::Maze;
                    }
                }
                SystemState::Maze => {
                    /* MAZE */
                    if self.wait_for_packet(145.into()).dat1() == 1 {
                        self.state = SystemState::Sos;
                        break;
                    }

                    if self.wait_for_packet(146.into()).dat1() == 1 {
                        self.state = SystemState::Idle;
                        break;
                    }

                    self.wait_for_packet(164.into());

                    let colours = Vec::new();

                    self.sensor_positions.iter().for_each(|sensor_pos| {
                        colours.push(
                            maze.get_colour_from_coord(sensor_pos.0, sensor_pos.1)
                                .expect("FATAL: colour in maze not found"),
                        )
                    });
                }
                SystemState::Sos => todo!(),
            }
        }
    }
}

impl BufferUser for Ss {
    fn write(&mut self, data: [u8; 4]) {
        self.out_buffer.lock().unwrap().write(data.into());
    }

    fn read(&mut self) -> Option<Packet> {
        self.in_buffer.lock().unwrap().read()
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
