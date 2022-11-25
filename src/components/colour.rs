#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colour {
    White = 0b000,
    Red = 0b001,
    Green = 0b010,
    Blue = 0b011,
    Black = 0b100,
}

impl From<char> for Colour {
    fn from(char: char) -> Self {
        match char.to_ascii_uppercase() {
            'W' => Colour::White,
            'R' => Colour::Red,
            'G' => Colour::Green,
            'B' => Colour::Blue,
            'N' => Colour::Black,
            _ => panic!("Expected values in [W, R, G, B, N]"),
        }
    }
}

impl From<u8> for Colour {
    fn from(number: u8) -> Self {
        match number {
            0b000 => Colour::White,
            0b001 => Colour::Red,
            0b010 => Colour::Green,
            0b011 => Colour::Blue,
            0b100 => Colour::Black,
            _ => panic!("Invalid conversion to colour"),
        }
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colour::Red => write!(f, "Red"),
            Colour::White => write!(f, "White"),
            Colour::Green => write!(f, "Green"),
            Colour::Blue => write!(f, "Blue"),
            Colour::Black => write!(f, "Black"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Colours {
    colours: [Colour; 5],
    index: usize,
}

impl From<u16> for Colours {
    fn from(colour_word: u16) -> Self {
        Colours {
            colours: [
                Colour::from(((colour_word & 0b0111000000000000) >> 12) as u8),
                Colour::from(((colour_word & 0b0000111000000000) >> 9) as u8),
                Colour::from(((colour_word & 0b0000000111000000) >> 6) as u8),
                Colour::from(((colour_word & 0b0000000000111000) >> 3) as u8),
                Colour::from(((colour_word & 0b0000000000000111) >> 0) as u8),
            ],
            index: 0,
        }
    }
}

impl Colours {
    pub fn new() -> Self {
        Self {
            colours: [Colour::White; 5],
            index: 0,
        }
    }

    pub fn all_white(&self) -> bool {
        for colour in self.colours {
            if colour != Colour::White {
                return false;
            }
        }

        true
    }
}

impl Iterator for Colours {
    type Item = Colour;

    fn next(&mut self) -> Option<Colour> {
        let result = match self.index {
            0 => Some(self.colours[0]),
            1 => Some(self.colours[1]),
            2 => Some(self.colours[2]),
            3 => Some(self.colours[3]),
            4 => Some(self.colours[4]),
            _ => None,
        };
        self.index += 1;
        result
    }
}
