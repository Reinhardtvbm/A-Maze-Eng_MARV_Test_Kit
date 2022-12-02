use super::comm_port::ControlByte;

#[derive(Clone, Copy, Debug)]
pub struct Packet {
    bytes: [u8; 4],
}

impl Packet {
    pub fn new(control_byte: u8, dat1: u8, dat0: u8, dec: u8) -> Self {
        Self {
            bytes: [control_byte, dat1, dat0, dec],
        }
    }

    // pub fn reset(&mut self) {
    //     self.bytes = [0; 4];
    // }

    pub fn control_byte(&self) -> ControlByte {
        ControlByte::from(self.bytes[0])
            .unwrap_or_else(|_| panic!("{} is not a valid control byte", self.bytes[0]))
    }

    pub fn dat1(&self) -> u8 {
        self.bytes[1]
    }

    pub fn dat0(&self) -> u8 {
        self.bytes[2]
    }

    pub fn dec(&self) -> u8 {
        self.bytes[3]
    }
}

impl From<[u8; 4]> for Packet {
    fn from(bytes: [u8; 4]) -> Self {
        Self { bytes }
    }
}
