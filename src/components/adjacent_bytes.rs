/// a wrapper around `(u8, u8)`. `AdjacentBytes` represents axactly that, two adjacent
/// bytes.
pub struct AdjacentBytes(u8, u8);

impl AdjacentBytes {
    /// use to create a new instance of the struct. `byte1` is the
    /// most significant / high byte, and `byte2` is the least significant
    /// or low byte
    pub fn make(byte1: u8, byte2: u8) -> Self {
        Self(byte1, byte2)
    }

    /// returns the most signigicant byte
    pub fn msb(&self) -> u8 {
        self.0
    }

    /// returns the least significant byte
    pub fn lsb(&self) -> u8 {
        self.1
    }
}

impl From<u16> for AdjacentBytes {
    /// use to convert a `u16` to `AdjacentBytes`.
    /// how: `let adj_bytes: AdjacentBytes = AdjacentBytes::from(80_u16);`
    /// or more idiomatically:
    ///     `let adj_bytes: AdjacentBytes = 80.into()`
    /// since the trait `Into` is automatically implemented when `From` is
    /// implemented, but the second way requires explicit type annotation
    fn from(integer: u16) -> Self {
        let [msb, lsb] = integer.to_be_bytes();
        AdjacentBytes(msb, lsb)
    }
}

impl From<AdjacentBytes> for u16 {
    /// see `From<u16> for AdjacentBytes`, use here is the same, but for
    /// conversion in the other direction.
    fn from(number: AdjacentBytes) -> Self {
        ((number.0 as u16) << 8) + number.1 as u16
    }
}
