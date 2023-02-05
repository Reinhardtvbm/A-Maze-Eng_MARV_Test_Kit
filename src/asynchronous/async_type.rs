use crate::components::buffer::Buffer;
use std::sync::{Arc, Mutex};

pub type PositionsEndpoint = Arc<Mutex<Buffer<[(f32, f32); 5]>>>;
