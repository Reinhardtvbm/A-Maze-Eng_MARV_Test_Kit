use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::components::packet::Packet;

use super::comm_port::ControlByte;

pub type SharedBuffer = Arc<Mutex<Buffer>>;

/// Get is a trait that was created to access the internal value of a SharedBuffer more easily,
/// by simply calling `get()` as opposed to `as_ref().borrow()`
pub trait Get {
    type Val;

    /// get the value immutably
    fn get(&self) -> &Self::Val;
    /// get the value mutably
    fn get_mut(&self) -> &mut Self::Val;
}

/// A wrapper struct for FILO / queue based buffer
/// a buffer. _Could become replaced by channels_ if
/// multithreading is implemented
#[derive(Debug, Clone)]
pub struct Buffer(VecDeque<Packet>);

impl Buffer {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    /// write/push to front of the queue
    pub fn write(&mut self, packet: Packet) {
        self.0.push_front(packet);
    }

    /// read the value at the back of the queue and remove it
    /// from the Buffer
    pub fn read(&mut self) -> Option<Packet> {
        self.0.pop_back()
    }

    /// read the value at the back of the queue without changing its
    /// contents
    pub fn peek(&self) -> Option<&Packet> {
        self.0.back()
    }

    pub fn empty(&self) -> bool {
        self.0.len() == 0
    }
}

/// A trait which defines the shared write and read
/// behaviours of the three subsystems `Snc`, `Mdps`, and
/// `Ss`.
///
/// Each struct will write to the buffers of the other two,
/// and in the case that they are using the serial port, that
/// will be read from / written to.
pub trait BufferUser {
    fn write(&mut self, data: [u8; 4]);
    fn read(&mut self) -> Packet;
    fn wait_for_packet(&mut self, control_byte: ControlByte) -> Packet;
}
