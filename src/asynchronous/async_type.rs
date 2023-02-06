use crate::components::{buffer::Buffer, packet::Packet};
use std::sync::{Arc, Mutex};

pub type PositionsEndpoint = Arc<Mutex<Buffer<[(f32, f32); 5]>>>;
pub type PacketsEndpoint = Arc<Mutex<Buffer<Packet>>>;
