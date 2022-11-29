use std::fmt::{self, Debug};

use serialport::SerialPort;

use super::packet::Packet;

#[derive(Debug)]
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
    pub fn read(&mut self) -> Result<Packet, ComPortError> {
        let mut buffer = [0_u8; 4];

        if self.serial_port.read(&mut buffer).is_ok() {
            Ok(Packet::from(buffer))
        } else {
            Err(ComPortError::ReadFail)
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

#[derive(PartialEq, Eq, Debug)]
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
    SosSpeed,
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
            228 => Ok(Self::SosSpeed),
            _ => Err(()),
        }
    }

    // pub fn to(byte: Self) -> Result<u8, ()> {
    //
    // }
}

impl fmt::Debug for ComPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serial_port.name().unwrap())
    }
}

impl From<ControlByte> for u8 {
    fn from(p: ControlByte) -> Self {
        match p {
            ControlByte::IdleButton => 16,
            ControlByte::Calibrated => 112,
            ControlByte::CalibrateOperationalVelocity => 96,
            ControlByte::CalibrateBatteryLevel => 97,
            ControlByte::CalibrateColours => 113,
            ControlByte::CalibrateButton => 80,
            ControlByte::MazeClapSnap => 145,
            ControlByte::MazeButton => 146,
            ControlByte::MazeNavInstructions => 147,
            ControlByte::MazeBatteryLevel => 161,
            ControlByte::MazeRotation => 162,
            ControlByte::MazeSpeeds => 163,
            ControlByte::MazeDistance => 164,
            ControlByte::MazeEndOfMaze => 179,
            ControlByte::MazeColours => 177,
            ControlByte::MazeIncidence => 178,
            ControlByte::SosSpeed => 228,
        }
    }
}
