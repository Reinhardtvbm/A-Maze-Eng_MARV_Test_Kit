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

#[derive(Debug, Clone)]
pub struct Buffer {
    heap: VecDeque<Packet>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            heap: VecDeque::new(),
        }
    }

    pub fn write(&mut self, packet: Packet) {
        self.heap.push_front(packet);

        println!("writing to buffer");
    }

    pub fn read(&mut self) -> Option<Packet> {
        self.heap.pop_back()
    }
}
