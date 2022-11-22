use crate::components::{
    colour::{Colour, Colours},
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
    colours: Colours,
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

    fn handle_incidence_with_line(
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

        match colour {
            Colour::Red | Colour::Green => self.green_encounter(incidence, side),
            Colour::Black | Colour::Blue => self.blue_encounter(incidence, side),
            _ => {}
        }
    }

    pub fn compute_output(&mut self, packets: [Packet; 5]) {
        let working_data = Self::parse_packets(packets);

        match self.current_state {
            NavConState::Forward => {
                if working_data.colours.all_white() {
                    // if all the sensors see white then
                    // MARV can continue going forward
                    return;
                } else {
                    for (index, colour) in working_data.colours.enumerate() {
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
            NavConState::Reverse => todo!(),
            NavConState::Stop => todo!(),
            NavConState::RotateLeft => todo!(),
            NavConState::RotateRight => todo!(),
        }
    }
}
