use std::cell::RefCell;
use std::rc::Rc;

use crate::components::buffer::{Buffer, SharedBuffer};
use crate::subsystems::mdps::mdps::Mdps;
use crate::subsystems::snc::snc::Snc;
use crate::subsystems::ss::ss::Ss;

/*
The bread and butter of the program:
    will emulate the maze robot
*/

#[derive(Debug)]
pub struct System {
    pub snc: Snc,
    pub ss: Ss,
    pub mdps: Mdps,
    pub shared_buffer: SharedBuffer,
}

impl System {
    pub fn new() -> Self {
        let shared_buf = Rc::new(RefCell::new(Buffer::new()));

        Self {
            snc: Snc::new(&shared_buf, false),
            ss: Ss {},
            mdps: Mdps {},
            shared_buffer: shared_buf,
        }
    }

    pub fn run(&mut self) {
        for _i in 0..5 {
            self.snc.run();
        }
    }
}
