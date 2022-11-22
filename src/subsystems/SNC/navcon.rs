use crate::components::colour::{Colour, Colours};

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

struct NavCon {
    current_state: NavConState,
    previous_state: NavConState,
    next_state: NavConState,
    incidence_side: Side,
    reference_distance: u16,
    previously_encountered_colour: Colour,
    input_colours: Colours,
    input_incidence: u8,
    input_distance: u16,
    output_speed_left: u8,
    output_speed_right: u8,
    output_rotation: u16,
}

impl NavCon {
    pub fn new() -> Self {
        Self {
            current_state: NavConState::Forward,
            previous_state: NavConState::Forward,
            next_state: NavConState::Forward,
            incidence_side: Side::Left,
            input_colours: Colours::new(),
            input_incidence: 0 as u8,
            output_speed_left: 0 as u8,
            output_speed_right: 0 as u8,
            output_rotation: 0 as u16,
            reference_distance: 0 as u16,
            input_distance: 0 as u16,
            previously_encountered_colour: Colour::White,
        }
    }

    pub fn update_input(&mut self, colours: Colours, incidence: u8) {
        self.input_colours = colours;
        self.input_incidence = incidence;
    }

    fn handle_indidence_with_line(&mut self, colour: Colour) {
        if self.input_distance - self.reference_distance > 65 {
            self.output_rotation = 5;
        } else {
            self.output_rotation = self.input_incidence as u16;
        }

        match colour {
            Colour::Red | Colour::Green => {
                if self.input_incidence <= 5 {
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
