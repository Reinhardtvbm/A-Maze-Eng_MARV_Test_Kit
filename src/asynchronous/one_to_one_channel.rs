use std::{
    fmt,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::components::buffer::Buffer;

use super::channel_err::ChannelRecErr;

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

        if self.endpoint.lock().unwrap().is_empty() {
            self.endpoint.lock().unwrap().write(data);
        } else {
            std::thread::sleep(Duration::from_nanos(100));
        }
    }

    /// wait for data to be present in the buffer, before returning
    /// the data first in the `Buffer`'s queue
    pub fn receive(&mut self) -> T {
        loop {
            if let Ok(data) = self.try_receive() {
                return data;
            }
        }
    }

    /// checks if there is data in the origin buffer, and returns it if
    /// there is
    pub fn try_receive(&mut self) -> Result<T, ChannelRecErr> {
        if self.origin.lock().unwrap().is_empty() {
            Err(ChannelRecErr::NoData)
        } else {
            Ok(self.origin.lock().unwrap().read().unwrap())
        }
    }
}
