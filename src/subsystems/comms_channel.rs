use std::time::Duration;

use crossbeam::channel::{Receiver, Sender, TryRecvError, TrySendError};

use crate::components::{buffer::Buffer, packet::Packet};

#[derive(Debug)]
pub struct CommsChannel {
    out_data: (Sender<Packet>, Sender<Packet>),
    in_data: (Receiver<Packet>, Receiver<Packet>),
    in_buffer: Buffer<Packet>,
    name: String,
}

impl CommsChannel {
    pub fn new(
        out_data: (Sender<Packet>, Sender<Packet>),
        in_data: (Receiver<Packet>, Receiver<Packet>),
        name: &str,
    ) -> Self {
        Self {
            out_data,
            in_data,
            in_buffer: Buffer::new(),
            name: String::from(name),
        }
    }

    pub fn send(&self, packet: Packet) {
        self.out_data
            .0
            .send(packet)
            .unwrap_or_else(|_| panic!("{} could not send {:?}", self.name, packet));

        self.out_data
            .1
            .send(packet)
            .unwrap_or_else(|_| panic!("{} could not send {:?}", self.name, packet));

        println!("{} sent {:?}", self.name, packet);
    }

    pub fn receive(&mut self) -> Packet {
        if self.in_buffer.empty() {
            println!("{} waiting for data...", self.name);
            self.wait_for_data();
        }

        let received_data = self.in_buffer.read().unwrap();

        println!("{} got {:?}", self.name, received_data);

        received_data
    }

    fn try_send(&self, channel: ChannelNo, msg: Packet) -> Result<(), TrySendError<Packet>> {
        match channel {
            ChannelNo::One => self.out_data.0.try_send(msg),
            ChannelNo::Two => self.out_data.1.try_send(msg),
        }
    }

    fn try_receive(&self, channel: ChannelNo) -> Result<Packet, TryRecvError> {
        match channel {
            ChannelNo::One => self.in_data.0.try_recv(),
            ChannelNo::Two => self.in_data.1.try_recv(),
        }
    }

    fn wait_for_data(&mut self) {
        while self.in_buffer.empty() {
            // attempt to get data from both channels

            // get all available data on channel one, NOTE: while
            while self.in_data.0.len() > 0 {
                self.in_buffer.write(self.in_data.0.recv().unwrap());
            }

            // get all available data on channel two, NOTE: while
            while self.in_data.1.len() > 0 {
                self.in_buffer.write(self.in_data.1.recv().unwrap());
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.in_buffer.empty()
    }

    pub fn peek(&self) -> Option<&Packet> {
        self.in_buffer.peek()
    }
}

enum ChannelNo {
    One,
    Two,
}
