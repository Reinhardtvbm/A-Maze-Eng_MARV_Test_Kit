use std::rc::Rc;

use crate::components::buffer::SharedBuffer;

#[derive(Debug)]
pub struct Mdps {
    read_buffer: SharedBuffer,
    write_buffers: [SharedBuffer; 2],
}

impl Mdps {
    pub fn new(w_buffers: [&SharedBuffer; 2], r_buffer: &SharedBuffer) -> Self {
        Self {
            read_buffer: Rc::clone(r_buffer),
            write_buffers: [Rc::clone(w_buffers[0]), Rc::clone(w_buffers[1])],
        }
    }
}
