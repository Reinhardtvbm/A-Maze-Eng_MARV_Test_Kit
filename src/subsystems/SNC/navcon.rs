use iced::futures::future::Map;

use crate::components::{
    adjacent_bytes::AdjacentBytes,
    colour::{Colour, Colours},
    comm_port::ControlByte,
    constants::Constants,
    packet::Packet,
};

enum NavConState {
    Forward,
    Reverse,
    Stop,
    RotateLeft,
    RotateRight,
}

enum Side {
    Left,
    Right,
}

struct WorkingData {
    colours: [Colour; 5],
    incidence: u8,
    distance: u16,
    speed_left: u8,
    speed_right: u8,
    rotation: u8,
}

struct NavCon {
    current_state: NavConState,
    previous_state: NavConState,
    next_state: NavConState,
    incidence_side: Side,
    previously_encountered_colour: Colour,
    output_rotation: u16,
    reference_distance: u16,
}

impl NavCon {
    pub fn new() -> Self {
        Self {
            current_state: NavConState::Forward,
            previous_state: NavConState::Forward,
            next_state: NavConState::Forward,
            incidence_side: Side::Left,
            output_rotation: 0 as u16,
            reference_distance: 0 as u16,
            previously_encountered_colour: Colour::White,
        }
    }

    fn parse_packets(packets: [Packet; 5]) -> WorkingData {
        WorkingData {
            colours: todo!(),
            incidence: todo!(),
            distance: todo!(),
            speed_left: todo!(),
            speed_right: todo!(),
            rotation: todo!(),
        }
    }

    fn green_encounter(&mut self, incidence: u8, side: Side) {
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
        }
    }

    fn blue_encounter(&mut self, incidence: u8, side: Side) {
        self.previous_state = NavConState::Forward;
        self.current_state = NavConState::Stop;
        self.next_state = NavConState::RotateRight;

        self.output_rotation = match side {
            Side::Left => 90 - incidence as u16,
            Side::Right => 90 + incidence as u16,
        }
    }

    fn handle_indidence_with_line(
        &mut self,
        incidence: u8,
        distance: u16,
        colour: Colour,
        side: Side,
    ) {
        if distance - self.reference_distance > Constants::inter_sensor_distance() {
            self.output_rotation = 5;
            return;
        }

        let add_to_rotation = match side {
            Side::Left => -1 * incidence as i16,
            Side::Right => incidence as i16,
        };

        match colour {
            Colour::Red | Colour::Green => {
                if incidence <= 5 {
                    return;
                }
                self.previously_encountered_colour = colour;
            }
            Colour::Black | Colour::Blue => {
                if self.previously_encountered_colour != Colour::Blue {
                    self.output_rotation += 90;
                } else {
                    self.output_rotation += 180;
                }
                self.previously_encountered_colour = Colour::Blue;
            }
            _ => {}
        }

        if let Side::Left = self.incidence_side {
            self.next_state = NavConState::RotateRight
        } else {
            self.next_state = NavConState::RotateRight
        }
    }

    pub fn compute_output(&mut self) {
        match self.current_state {
            NavConState::Forward => {
                if self.input_colours.all_white() {
                    // if all the sensors see white then
                    // MARV can continue going forward
                    return;
                } else {
                    for (index, colour) in self.input_colours.enumerate() {
                        if colour != Colour::White {
                            match index {
                                0 => {
                                    self.incidence_side = Side::Left;
                                    self.reference_distance = self.input_distance;
                                }
                                1 => self.handle_indidence_with_line(colour),
                                3 => self.handle_indidence_with_line(colour),
                                4 => {
                                    self.incidence_side = Side::Right;
                                    self.reference_distance = self.input_distance;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            NavConState::Reverse => todo!(),
            NavConState::Stop => todo!(),
            NavConState::RotateLeft => todo!(),
            NavConState::RotateRight => todo!(),
        }
    }
}
