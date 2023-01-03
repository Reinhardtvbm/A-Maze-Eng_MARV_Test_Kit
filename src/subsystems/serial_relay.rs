use std::sync::Arc;

use crate::components::{
    buffer::{BufferUser, SharedBuffer},
    comm_port::ComPort,
    packet::Packet,
};

pub struct SerialRelay {
    port: ComPort,
    in_buffer: SharedBuffer,
    out_buffer: SharedBuffer,
}

impl SerialRelay {
    pub fn new(out_buffer: &SharedBuffer, in_buffer: &SharedBuffer, com_no: String) -> Self {
        Self {
            port: ComPort::new(com_no, 19200),
            in_buffer: Arc::clone(in_buffer),
            out_buffer: Arc::clone(out_buffer),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(packet) = self.port.read() {
                self.write(packet.into());
            }

            if let Some(packet) = self.read() {
                self.port.write(&packet.into()).unwrap()
            }
        }
    }
}

impl BufferUser for SerialRelay {
    /// writes to the output buffer
    fn write(&mut self, data: [u8; 4]) {
        self.out_buffer.lock().unwrap().write(data.into());
    }

    /// reads from the input buffer
    fn read(&mut self) -> Option<Packet> {
        self.in_buffer.lock().unwrap().read()
    }

    fn wait_for_packet(
        &mut self,
        control_byte: crate::components::comm_port::ControlByte,
    ) -> Packet {
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
