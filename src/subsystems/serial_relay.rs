use crate::{
    asynchronous::one_to_many_channel::OTMChannel,
    components::{comm_port::ComPort, packet::Packet},
};

pub struct SerialRelay {
    port_number: String,
    port: ComPort,
    channel: OTMChannel<Packet>,
}

impl SerialRelay {
    pub fn new(channel: OTMChannel<Packet>, com_no: &str) -> Self {
        Self {
            port_number: String::from(com_no),
            port: ComPort::new(String::from(com_no), 19200),
            channel,
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(com_port_data) = self.port.try_read() {
                self.channel.send(com_port_data);
            }

            if let Ok(channel_data) = self.channel.try_receive() {
                self.port
                    .write(&mut channel_data.into())
                    .unwrap_or_else(|_| {
                        panic!(
                            "FATAL: could not write to {} port (COM{})",
                            self.channel.name(),
                            self.port_number
                        )
                    });
            }
        }
    }
}
