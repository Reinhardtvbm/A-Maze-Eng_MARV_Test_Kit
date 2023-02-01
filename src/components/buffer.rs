use crate::components::comm_port::ControlByte;
use crate::components::packet::Packet;
use std::collections::VecDeque;

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
pub struct Buffer<T>(VecDeque<T>);

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    /// write/push to front of the queue
    pub fn write(&mut self, data: T) {
        self.0.push_front(data);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// read the value at the back of the queue and remove it
    /// from the Buffer
    pub fn read(&mut self) -> Option<T> {
        self.0.pop_back()
    }

    /// read the value at the back of the queue without changing its
    /// contents
    pub fn peek(&self) -> Option<&T> {
        self.0.back()
    }

    pub fn empty(&self) -> bool {
        self.0.is_empty()
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
