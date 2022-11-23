use crate::components::packet::Packet;
use std::sync::Arc;

#[derive(Debug)]
pub struct Buffer {
    heap: Vec<Packet>,
}

impl Buffer {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }
}
