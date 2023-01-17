use crate::components::{
    buffer::BufferUser,
    comm_port::{ComPort, ControlByte},
    packet::Packet,
};

use super::comms_channel::CommsChannel;

pub struct SerialRelay {
    port: ComPort,
    comms: CommsChannel,
}

impl SerialRelay {
    pub fn new(comms: CommsChannel, com_no: String) -> Self {
        Self {
            port: ComPort::new(com_no, 19200),
            comms,
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(packet) = self.port.read() {
                self.write(packet.into());
            }

            let write_to_port = match self.comms.is_empty() {
                true => None,
                false => Some(self.read()),
            };

            if let Some(packet) = write_to_port {
                self.port
                    .write(&packet.into())
                    .expect("FATAL: Serial relay could not write to serial port");
            }
        }
    }
}

impl BufferUser for SerialRelay {
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
