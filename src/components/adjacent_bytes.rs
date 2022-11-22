pub struct AdjacentBytes(u8, u8);

impl AdjacentBytes {
    pub fn make(byte1: u8, byte2: u8) -> Self {
        Self(byte1, byte2)
    }

    pub fn get_msb(&self) -> u8 {
        self.0
    }

    pub fn get_lsb(&self) -> u8 {
        self.1
    }
}

impl From<u16> for AdjacentBytes {
    fn from(integer: u16) -> Self {
        AdjacentBytes(((integer & 0xFF00) >> 8) as u8, (integer & 0x00FF) as u8)
    }
}

impl From<AdjacentBytes> for u16 {
    fn from(number: AdjacentBytes) -> Self {
        return ((number.0 as u16) << 8) + number.1 as u16;
    }
}
