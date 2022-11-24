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
}

impl System {
    pub fn new() -> Self {
        let snc_buffer = Rc::new(RefCell::new(Buffer::new()));
        let ss_buffer = Rc::new(RefCell::new(Buffer::new()));
        let mdps_buffer = Rc::new(RefCell::new(Buffer::new()));

        Self {
            snc: Snc::new([&ss_buffer, &mdps_buffer], &snc_buffer, false),
            ss: Ss::new([&snc_buffer, &mdps_buffer], &ss_buffer),
            mdps: Mdps::new([&ss_buffer, &snc_buffer], &mdps_buffer),
        }
    }

    pub fn run(&mut self) {
        for _i in 0..3 {
            self.snc.run();
        }
    }
}
