use crossbeam::channel::{Receiver, Sender};

use crate::components::{buffer::Buffer, packet::Packet};

#[derive(Debug)]
pub struct CommsChannel {
    out_data: (Sender<Packet>, Sender<Packet>),
    in_data: (Receiver<Packet>, Receiver<Packet>),
    in_buffer: Buffer,
}

impl CommsChannel {
    pub fn new(
        out_data: (Sender<Packet>, Sender<Packet>),
        in_data: (Receiver<Packet>, Receiver<Packet>),
    ) -> Self {
        Self {
            out_data,
            in_data,
            in_buffer: Buffer::new(),
        }
    }

    pub fn send(&self, p: Packet) {
        println!("sending {:?}", p);

        self.out_data
            .0
            .send(p)
            .unwrap_or_else(|_| panic!("FATAL: comms channel {:?} failed to send data", self));

        self.out_data
            .1
            .send(p)
            .unwrap_or_else(|_| panic!("FATAL: comms channel {:?} failed to send data", self));
    }

    pub fn receive(&mut self) -> Packet {
        while self.in_buffer.empty() {
            if self.in_data.0.len() > 0 {
                self.in_buffer.write(self.in_data.0.recv().unwrap());
            }

            if self.in_data.1.len() > 0 {
                self.in_buffer.write(self.in_data.1.recv().unwrap());
            }
        }
        let p = self.in_buffer.read().unwrap();

        println!("got {:?}", p);
        p
    }

    pub fn is_empty(&self) -> bool {
        self.in_buffer.empty()
    }

    pub fn peek(&self) -> Option<&Packet> {
        self.in_buffer.peek()
    }
}
