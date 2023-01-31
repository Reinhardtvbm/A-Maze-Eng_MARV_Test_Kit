use std::{
    fmt,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::components::buffer::Buffer;

pub struct OTOChannel<T>
where
    T: Copy + fmt::Debug,
{
    name: String,
    origin: Arc<Mutex<Buffer<T>>>,
    endpoint: Arc<Mutex<Buffer<T>>>,
}

impl<T: Copy + fmt::Debug> OTOChannel<T> {
    pub fn new(
        name: &str,
        origin: &Arc<Mutex<Buffer<T>>>,
        endpoint: &Arc<Mutex<Buffer<T>>>,
    ) -> Self {
        Self {
            name: String::from(name),
            origin: Arc::clone(origin),
            endpoint: Arc::clone(endpoint),
        }
    }

    pub fn send(&self, data: T) {
        println!("{} sending {:?}", self.name, data);

        self.endpoint.lock().unwrap().write(data);
    }

    pub fn receive(&self) -> T {
        loop {
            if !self.origin.lock().unwrap().empty() {
                return self.origin.lock().unwrap().read().unwrap();
            }

            std::thread::sleep(Duration::from_nanos(100));
        }
    }
}
