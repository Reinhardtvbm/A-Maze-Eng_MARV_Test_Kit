use std::{
    cell::{Ref, RefCell, RefMut},
    collections::VecDeque,
    rc::Rc,
};

use crate::components::packet::Packet;

pub type SharedBuffer = Rc<RefCell<Buffer>>;

pub trait Get {
    type Val;

    fn get(&self) -> Ref<'_, Self::Val>;
    fn get_mut(&self) -> RefMut<'_, Self::Val>;
}

impl Get for SharedBuffer {
    type Val = Buffer;

    fn get(&self) -> Ref<'_, Self::Val> {
        self.as_ref().borrow()
    }

    fn get_mut(&self) -> RefMut<'_, Self::Val> {
        self.as_ref().borrow_mut()
    }
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

    pub fn write(&mut self, packet: Packet) {
        self.0.push_front(packet);

        println!("writing to buffer");
    }

    pub fn read(&mut self) -> Option<Packet> {
        self.0.pop_back()
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
    fn write(&mut self, data: &mut [u8; 4]);
    fn read(&mut self) -> Option<Packet>;
}
