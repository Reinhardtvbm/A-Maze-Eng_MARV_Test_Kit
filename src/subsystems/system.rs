use std::sync::Arc;

use crate::components::buffer::Buffer;
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
    pub shared_buffer: Arc<Buffer>,
}

impl System {
    pub fn new() -> Self {
        let shared_buf = Arc::new(Buffer::new());

        Self {
            snc: Snc::new(&shared_buf, false),
            ss: Ss {},
            mdps: Mdps {},
            shared_buffer: shared_buf,
        }
    }
}
