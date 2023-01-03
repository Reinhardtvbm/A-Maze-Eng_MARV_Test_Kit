use std::sync::Arc;

use crate::components::{
    buffer::{BufferUser, SharedBuffer},
    colour::Colours,
    packet::Packet,
};

#[derive(Debug)]
pub struct Ss {
    /// the x-y coordinates of each sensor, these will be
    /// updated using the distance and rotation measurements
    /// from the MDPS
    sensor_positions: [(f32, f32); 5],
    /// the colour that each sensor senses, will get via the
    /// `get_colour_from_coord` method in the `maze` module
    sensor_colours: Colours,
    /// A shared buffer of type Rc<RefCell<_>>
    /// which is written to by the other two subsystems
    in_buffer: SharedBuffer,
    /// The shared buffers of the other two subsystems
    /// for the MDPS to send its data to
    out_buffer: SharedBuffer,
}

impl Ss {
    pub fn new(
        out_buffer: &SharedBuffer,
        in_buffer: &SharedBuffer,
        init_sensor_pos: [(f32, f32); 5],
    ) -> Self {
        Self {
            sensor_colours: Colours::new(),
            sensor_positions: init_sensor_pos,
            in_buffer: Arc::clone(in_buffer),
            out_buffer: Arc::clone(out_buffer),
        }
    }

    pub fn run(&mut self) {}
}

impl BufferUser for Ss {
    fn write(&mut self, data: [u8; 4]) {
        self.out_buffer.lock().unwrap().write(data.into());
    }

    fn read(&mut self) -> Option<Packet> {
        self.in_buffer.lock().unwrap().read()
    }
}
