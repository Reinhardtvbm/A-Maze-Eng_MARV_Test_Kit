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

#[derive(Debug)]
pub struct Buffer {
    heap: VecDeque<Vec<Packet>>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            heap: VecDeque::new(),
        }
    }

    pub fn write(&mut self, packet: Packet) {
        self.heap.push_front(vec![packet; 2]);

        println!("writing to buffer");
    }

    pub fn read(&mut self) -> Result<Packet, ()> {
        let return_packet = self.heap.back_mut().expect("buffer empty :(").remove(0);

        if self.heap.back().expect("buffer empty :(").len() == 0 {
            self.heap.pop_back();
        }

        println!("read from buffer");

        return Ok(return_packet);
    }
}
