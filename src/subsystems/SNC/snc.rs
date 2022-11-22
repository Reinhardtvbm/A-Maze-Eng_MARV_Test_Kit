use crate::components::colour::{Colour, Colours};
use crate::components::comm_port::{ComPort, ControlByte};

enum NavInstruction {
    Forward,
    Reverse,
    RotateLeft,
    RotateRight,
}





struct StateCon {}

pub struct Snc {
    comm_port: ComPort,
    input_data: NavConInputData,
    output_data: NavConOutputData,
    prev_instruction: NavInstruction,
    next_instruction: NavInstruction,
    motor_instruction: NavInstruction,
}

impl Snc {
    pub fn new(comm_port_number: String, baud_rate: u32) -> Self {
        Self {
            comm_port: ComPort::new(comm_port_number, baud_rate),
            input_data: NavConInputData {
                colours: Colours::from(0),
                incidence: 0,
            },
                speed: None,
                rotation: None,
            },
            prev_instruction: NavInstruction::Forward,
            next_instruction: NavInstruction::Forward,
        }
    }

    pub fn update_input_data(&mut self, packet: [u8; 4]) -> Result<(), ()> {
        let control_byte_result = ControlByte::from(packet[0]);

        if control_byte_result.is_ok() {
            let control_byte = control_byte_result.unwrap();

            if control_byte == ControlByte::MazeColours {
                return Ok(self.input_data.colours =
                    Colours::from(((packet[1] as u16) << 8) + (packet[2] as u16)));
            } else if control_byte == ControlByte::MazeIncidence {
                return Ok(self.input_data.incidence = packet[1]);
            }
            Err(())
        } else {
            Err(())
        }
    }

    pub fn update_output_data(&mut self) {
        
    }
}