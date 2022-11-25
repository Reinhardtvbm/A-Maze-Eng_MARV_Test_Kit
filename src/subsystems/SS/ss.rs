use std::rc::Rc;

use crate::components::{
    buffer::{BufferUser, Get, SharedBuffer},
    comm_port::ComPort,
    packet::Packet,
};

#[derive(Debug)]
pub struct Ss {
    read_buffer: SharedBuffer,
    write_buffers: [SharedBuffer; 2],
    port: Option<ComPort>,
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
        }
    }
}

impl BufferUser for Ss {
    fn write(&mut self, data: &mut [u8; 4]) {
        if self.port.is_some() {
            self.port
                .as_mut()
                .unwrap()
                .write(&data)
                .expect("Could not write to port in Maze state");
        } else {
            let write_data = *data;

            self.write_buffers[0]
                .get_mut()
                .write(Packet::from(write_data));
            self.write_buffers[1]
                .get_mut()
                .write(Packet::from(write_data));
        }
    }

    fn read(&mut self) -> Option<Packet> {
        if self.port.is_some() {
            Some(
                self.port
                    .as_mut()
                    .unwrap()
                    .read()
                    .expect("Failed to read from port in Calibrate"),
            )
        } else {
            self.read_buffer.get_mut().read()
        }
    }
}
