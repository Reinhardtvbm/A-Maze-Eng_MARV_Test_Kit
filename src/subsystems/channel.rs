use std::{
    fmt,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::components::buffer::Buffer;

pub struct Channel<T>
where
    T: Copy + fmt::Debug,
{
    name: String,
    endpoints: Vec<Arc<Mutex<Buffer<T>>>>,
    origin: Arc<Mutex<Buffer<T>>>,
}

impl<T: Copy + fmt::Debug> Channel<T> {
    /// creates a `Channel<T>` without any endpoints
    pub fn new(name: &str, origin: &Arc<Mutex<Buffer<T>>>) -> Self {
        Self {
            name: String::from(name),
            endpoints: Vec::new(),
            origin: Arc::clone(origin),
        }
    }

    /// creates a `Channel<T>` with the provided endpoints
    pub fn with_endpoints(
        name: &str,
        origin: &Arc<Mutex<Buffer<T>>>,
        endpoints: Vec<&Arc<Mutex<Buffer<T>>>>,
    ) -> Self {
        Self {
            name: String::from(name),
            // map each endpoint data to an Arc<Mutex<endpoint>>
            endpoints: endpoints
                .into_iter()
                .map(|element| Arc::clone(element))
                .collect(),
            origin: Arc::clone(origin),
        }
    }

    /// adds a new endpoint to the `Channel`
    pub fn add_endpoint(&mut self, endpoint: &Arc<Mutex<Buffer<T>>>) {
        self.endpoints.push(Arc::clone(endpoint));
    }

    /// waits for a lock on each buffer's mutex, and writes `data` to each `Buffer<T>`
    pub fn send(&self, data: T) {
        println!("{} sending {:?}", self.name, data);

        self.endpoints.iter().for_each(|buffer| {
            buffer.lock().unwrap().write(data);
        });
    }

    /// wait for data to be present in the buffer, before returning
    /// the data first in the `Buffer`'s queue
    pub fn receive(&mut self) -> T {
        loop {
            if !self.origin.lock().unwrap().empty() {
                return self.origin.lock().unwrap().read().unwrap();
            }

            std::thread::sleep(Duration::from_micros(1));
        }
    }
}

impl<T: Copy + fmt::Debug> std::fmt::Debug for Channel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Channel")
            .field("origin", &self.name)
            .finish()
    }
}
