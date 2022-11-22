use serialport::SerialPort;

pub enum ComPortError {
    ReadFail,
    WriteFail,
}

pub struct ComPort {
    serial_port: Box<dyn SerialPort>,
}

impl ComPort {
    /// will create a new instance of an a-maze-eng-MARV com port
    pub fn new(comm_port_number: String, baud_rate: u32) -> Self {
        Self {
            serial_port: serialport::new(format!("COM{}", comm_port_number), baud_rate)
                .open()
                .expect("Failed to open port"),
        }
    }

    /// reads 4 bytes from the serial port
    pub fn read(&mut self) -> Result<[u8; 4], ComPortError> {
        let mut buffer = [0_u8; 4];

        if self.serial_port.read(&mut buffer).is_ok() {
            return Ok(buffer);
        } else {
            return Err(ComPortError::ReadFail);
        }
    }

    /// writes 4 bytes to the serial port
    pub fn write(&mut self, packet: &[u8; 4]) -> Result<(), ComPortError> {
        if self.serial_port.write(packet).is_ok() {
            Ok(())
        } else {
            Err(ComPortError::WriteFail)
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ControlByte {
    IdleButton,
    Calibrated,
    CalibrateOperationalVelocity,
    CalibrateBatteryLevel,
    CalibrateColours,
    CalibrateButton,
    MazeClapSnap,
    MazeButton,
    MazeNavInstructions,
    MazeBatteryLevel,
    MazeRotation,
    MazeSpeeds,
    MazeDistance,
    MazeEndOfMaze,
    MazeColours,
    MazeIncidence,
}

impl ControlByte {
    pub fn from(byte: u8) -> Result<Self, ()> {
        match byte {
            16 => Ok(Self::IdleButton),
            112 => Ok(Self::Calibrated),
            96 => Ok(Self::CalibrateOperationalVelocity),
            97 => Ok(Self::CalibrateBatteryLevel),
            80 => Ok(Self::CalibrateButton),
            113 => Ok(Self::CalibrateColours),
            145 => Ok(Self::MazeClapSnap),
            146 => Ok(Self::MazeButton),
            147 => Ok(Self::MazeNavInstructions),
            161 => Ok(Self::MazeBatteryLevel),
            162 => Ok(Self::MazeRotation),
            163 => Ok(Self::MazeSpeeds),
            164 => Ok(Self::MazeDistance),
            179 => Ok(Self::MazeEndOfMaze),
            177 => Ok(Self::MazeColours),
            178 => Ok(Self::MazeIncidence),
            _ => Err(()),
        }
    }

    pub fn to(byte: Self) -> Result<u8, ()> {
        match byte {
            ControlByte::IdleButton => Ok(16),
            ControlByte::Calibrated => Ok(112),
            ControlByte::CalibrateOperationalVelocity => Ok(96),
            ControlByte::CalibrateBatteryLevel => Ok(97),
            ControlByte::CalibrateColours => Ok(113),
            ControlByte::CalibrateButton => Ok(80),
            ControlByte::MazeClapSnap => Ok(145),
            ControlByte::MazeButton => Ok(146),
            ControlByte::MazeNavInstructions => Ok(147),
            ControlByte::MazeBatteryLevel => Ok(161),
            ControlByte::MazeRotation => Ok(162),
            ControlByte::MazeSpeeds => Ok(163),
            ControlByte::MazeDistance => Ok(164),
            ControlByte::MazeEndOfMaze => Ok(179),
            ControlByte::MazeColours => Ok(177),
            ControlByte::MazeIncidence => Ok(178),
            _ => Err(()),
        }
    }
}
