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

/// A simple struct that acts an an abstraction for
/// a buffer. _Could become replaced by channels_ if
/// multithreading is implemented
#[derive(Debug, Clone)]
pub struct Buffer {
    queue: VecDeque<Packet>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn write(&mut self, packet: Packet) {
        println!("\nbuffer: {:?} before write", self.queue);
        self.queue.push_front(packet);
        println!("\nbuffer: {:?} after write", self.queue);
    }

    pub fn read(&mut self) -> Option<Packet> {
        println!("\nbuffer: {:?} before read", self.queue);
        let p = self.queue.pop_back();

        println!("\nbuffer: {:?} after read", self.queue);

        p
    }
}

/// A trait which defines the shared write and read
/// behaviours of the three subsystems `Snc`, `Mdps`, and
/// `Ss`
pub trait BufferUser {
    fn write(&mut self, data: &mut [u8; 4]);
    fn read(&mut self) -> Option<Packet>;
}
