use std::rc::Rc;

use crate::components::{
    buffer::{BufferUser, Get, SharedBuffer},
    comm_port::{ComPort, ControlByte},
    packet::Packet,
    state::SystemState,
};

#[derive(Debug)]
pub struct Ss {
    read_buffer: SharedBuffer,
    write_buffers: [SharedBuffer; 2],
    port: Option<ComPort>,
    state: SystemState,
}

impl Ss {
    pub fn new(
        w_buffers: [&SharedBuffer; 2],
        r_buffer: &SharedBuffer,
        activate_port: bool,
    ) -> Self {
        let comm_port = match activate_port {
            true => Some(ComPort::new(String::from("69"), 19200)),
            false => None,
        };

        Self {
            read_buffer: Rc::clone(r_buffer),
            write_buffers: [Rc::clone(w_buffers[0]), Rc::clone(w_buffers[1])],
            port: comm_port,
            state: SystemState::Idle,
        }
    }

    pub fn run(&mut self) {
        match self.state {
            SystemState::Idle => {
                /* Idle things */
                println!("SS in idle");

                if let Some(packet) = self.read() {
                    if packet.control_byte() == ControlByte::IdleButton && packet.dat1() == 1 {
                        self.state = SystemState::Calibrate;
                    }
                }
            }
            SystemState::Calibrate => {
                println!("SS in calibrate");
                self.write(&mut [112, 0, 0, 0]);

                if let Some(packet) = self.read() {
                    if packet.control_byte() == ControlByte::CalibrateBatteryLevel {
                        self.write(&mut [113, 0, 0, 0]);
                    } else if packet.control_byte() == ControlByte::CalibrateButton
                        && packet.dat1() == 1
                    {
                        self.state = SystemState::Maze;
                    }
                }
            }
            SystemState::Maze => {
                println!("SS in maze");

                if let Some(packet) = self.read() {
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
                        ControlByte::MazeDistance => {
                            todo!();

                            /*
                                - get colours somehow from GUI
                                - test for end of maze, else:
                                    - write colours
                                    - write angle of incidence
                            */
                        }
                        _ => (),
                    }
                }
            }
            SystemState::Sos => todo!(),
        }
    }
}

impl BufferUser for Ss {
    fn write(&mut self, data: &mut [u8; 4]) {
        match self.port.as_mut() {
            Some(port) => port.write(data).expect("Could not write to port."),
            None => {
                println!("SS writing {:?}", data);

                let write_data = *data;

                self.write_buffers[0]
                    .get_mut()
                    .write(Packet::from(write_data));
                self.write_buffers[1]
                    .get_mut()
                    .write(Packet::from(write_data));
            }
        }
    }

    fn read(&mut self) -> Option<Packet> {
        println!("SS reading buffer");

        match self.port.as_mut() {
            Some(com_port) => Some(com_port.read().expect("Failed to read from port.")),
            None => self.read_buffer.get_mut().read(),
        }
    }
}
