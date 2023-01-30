use crate::components::{
    adjacent_bytes::AdjacentBytes,
    colour::{Colour, Colours},
    comm_port::ControlByte,
    constants::B_ISD,
    packet::Packet,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum NavConState {
    Forward,
    Reverse,
    Stop,
    RotateLeft,
    RotateRight,
}

#[derive(Debug)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
struct WorkingData {
    colours: Colours,
    incidence: u8,
    distance: u16,
}

#[derive(Debug)]
pub struct NavCon {
    current_state: NavConState,
    previous_state: NavConState,
    next_state: NavConState,
    previously_encountered_colour: Colour,
    pub output_rotation: u16,
    reference_distance: u16,
}

impl NavCon {
    pub fn new() -> Self {
        Self {
            current_state: NavConState::Forward,
            previous_state: NavConState::Forward,
            next_state: NavConState::Forward,
            output_rotation: 0,
            reference_distance: 0,
            previously_encountered_colour: Colour::White,
        }
    }

    fn parse_packets(packets: [Packet; 5]) -> WorkingData {
        let mut colours = Colours::new();
        let mut incidence = 0;
        let mut distance = 0;

        for packet in packets {
            match packet.control_byte() {
                ControlByte::MazeDistance => {
                    distance = u16::from(AdjacentBytes::make(packet.dat1(), packet.dat0()));
                }
                ControlByte::MazeColours => {
                    colours =
                        Colours::from(u16::from(AdjacentBytes::make(packet.dat1(), packet.dat0())));
                }
                ControlByte::MazeIncidence => {
                    incidence = packet.dat1();
                }
                _ => (),
            }
        }

        WorkingData {
            colours,
            incidence,
            distance,
        }
    }

    fn green_encounter(&mut self, incidence: u8, side: Side) {
        // println!("green");

        self.output_rotation = match incidence {
            0..=5 => return,
            6..=44 => incidence as u16,
            _ => 5,
        };

        self.previous_state = NavConState::Forward;
        self.current_state = NavConState::Stop;

        self.next_state = match side {
            Side::Left => NavConState::RotateLeft,
            Side::Right => NavConState::RotateRight,
        };

        self.previously_encountered_colour = Colour::Green;
    }

    fn blue_encounter(&mut self, incidence: u8, side: Side) {
        self.previous_state = NavConState::Forward;
        self.current_state = NavConState::Stop;
        self.next_state = NavConState::RotateRight;

        self.output_rotation = match side {
            Side::Left => 90 - incidence as u16,
            Side::Right => 90 + incidence as u16,
        };

        if self.previously_encountered_colour == Colour::Blue {
            self.output_rotation += 90;
        }

        self.previously_encountered_colour = Colour::Blue;
    }

    fn handle_incidence_with_line(
        &mut self,
        incidence: u8,
        distance: u16,
        colour: Colour,
        side: Side,
    ) {
        if distance as i16 - self.reference_distance as i16 > B_ISD as i16 {
            self.output_rotation = 5;
            return;
        }

        match colour {
            Colour::Red | Colour::Green => self.green_encounter(incidence, side),
            Colour::Black | Colour::Blue => self.blue_encounter(incidence, side),
            _ => {}
        }
    }

    pub fn get_state(&self) -> NavConState {
        self.current_state
    }

    pub fn compute_output(&mut self, packets: [Packet; 5]) {
        let working_data = Self::parse_packets(packets);

        match self.current_state {
            NavConState::Forward => {
                if !working_data.colours.all_white() {
                    for (index, colour) in working_data.colours.into_iter().enumerate() {
                        if colour != Colour::White {
                            match index {
                                1 => self.handle_incidence_with_line(
                                    working_data.incidence,
                                    working_data.distance,
                                    colour,
                                    Side::Left,
                                ),
                                3 => self.handle_incidence_with_line(
                                    working_data.incidence,
                                    working_data.distance,
                                    colour,
                                    Side::Right,
                                ),
                                0 | 4 => self.reference_distance = working_data.distance,
                                _ => {}
                            }
                        }
                    }
                }
            }
            NavConState::Reverse => {
                // until MARV has reversed for 6cm, keep reversing....

                //println!("{}", working_data.distance);
                if working_data.distance < 30 {
                    return;
                }

                self.previous_state = NavConState::Reverse;
                self.current_state = NavConState::Stop;
            }
            NavConState::Stop => {
                self.current_state = match self.previous_state {
                    NavConState::Forward => NavConState::Reverse,
                    _ => self.next_state,
                };
            }
            NavConState::RotateLeft => {
                self.current_state = NavConState::Forward;
            }
            NavConState::RotateRight => {
                self.current_state = NavConState::Forward;
            }
        }
    }
}
