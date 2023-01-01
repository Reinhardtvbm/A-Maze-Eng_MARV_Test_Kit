use std::{rc::Rc, sync::Arc, time::SystemTime};

use crate::components::{
    buffer::{BufferUser, Get, SharedBuffer},
    colour::Colours,
    comm_port::ComPort,
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
    read_buffer: SharedBuffer,
    /// The shared buffers of the other two subsystems
    /// for the MDPS to send its data to
    write_buffers: [SharedBuffer; 2],
    /// The serial port to write to if serial comms are
    /// being utilised
    port: Option<ComPort>,
}

impl Ss {
    pub fn new(
        w_buffers: (&SharedBuffer, &SharedBuffer),
        r_buffer: &SharedBuffer,
        activate_port: bool,
        init_sensor_pos: [(f32, f32); 5],
    ) -> Self {
        let comm_port = match activate_port {
            true => Some(ComPort::new(String::from("69"), 19200)),
            false => None,
        };

        Self {
            sensor_colours: Colours::new(),
            sensor_positions: init_sensor_pos,
            read_buffer: Arc::clone(r_buffer),
            write_buffers: [Arc::clone(w_buffers.0), Arc::clone(w_buffers.1)],
            port: comm_port,
        }
    }
}

impl BufferUser for Ss {
    fn write(&mut self, data: &mut [u8; 4]) {
        if let Some(com_port) = &mut self.port {
            com_port.write(data).expect("SS could not write to port.");
        }

        let write_data = *data;

        self.write_buffers
            .iter()
            .for_each(|buffer| buffer.lock().unwrap().write(write_data.into()));
    }

    fn read(&mut self) -> Option<Packet> {
        let port_data;
        let buffer_data = self.read_buffer.lock().unwrap().read();

        if let Some(com_port) = &mut self.port {
            port_data = com_port.read().expect("Failed to read from port.");

            // here we do a sanity check, just to make sure
            if buffer_data.unwrap() != port_data {
                panic!("FATAL: serial port data does not match buffer data");
            }
        }

        buffer_data
    }
}
