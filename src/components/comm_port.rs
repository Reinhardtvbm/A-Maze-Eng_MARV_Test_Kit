use std::{
    fmt::{self, Debug},
    time::Duration,
};

extern crate serialport;

use serialport::SerialPort;

use super::packet::Packet;

#[derive(Debug)]
pub enum ComPortError {
    ReadFail,
    WriteFail,
}

pub enum ComPortReadErr {
    NoData,
}

pub struct ComPort(Box<dyn SerialPort>);

impl ComPort {
    /// will create a new instance of an a-maze-eng-MARV com port
    pub fn new(comm_port_number: String, baud_rate: u32) -> Self {
        Self(
            serialport::new(format!("COM{}", comm_port_number), baud_rate)
                .open()
                .unwrap_or_else(|_| panic!("Failed to open COM{}", comm_port_number)),
        )
    }

    /// reads 4 bytes from the serial port
    pub fn read(&mut self) -> Result<Packet, ComPortError> {
        let mut buffer = [0; 4];

        if let Ok(no_bytes) = self.0.read(&mut buffer) {
            if no_bytes >= 4 {
                Ok(Packet::from(buffer))
            } else {
                Err(ComPortError::ReadFail)
            }
        } else {
            Err(ComPortError::ReadFail)
        }
    }

    /// writes 4 bytes to the serial port
    pub fn write(&mut self, packet: &[u8; 4]) -> Result<(), ComPortError> {
        if self.0.write(packet).is_ok() {
            Ok(())
        } else {
            Err(ComPortError::WriteFail)
        }
    }

    pub fn try_read(&mut self) -> Result<Packet, ComPortReadErr> {
        // NOTE: the unwrap() here means that before data is read from the
        // COM Port, we must know that there is a valid serial connection
        if self.0.bytes_to_read().unwrap() < 4 {
            // if there aren't enough bytes to read yet, then delay for 1us
            std::thread::sleep(Duration::from_micros(1));
            return Err(ComPortReadErr::NoData);
        }

        let mut bytes = [0u8; 4];

        if let Err(e) = self.0.read(&mut bytes) {
            panic!("FATAL: failed to read from serial port ({})", e);
        }

        Ok(bytes.into())
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
    SosClapSnap,
    Undefined,
}

impl fmt::Debug for ComPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.name().unwrap())
    }
}

impl From<u8> for ControlByte {
    fn from(byte: u8) -> Self {
        match byte {
            16 => Self::IdleButton,
            112 => Self::Calibrated,
            96 => Self::CalibrateOperationalVelocity,
            97 => Self::CalibrateBatteryLevel,
            80 => Self::CalibrateButton,
            113 => Self::CalibrateColours,
            145 => Self::MazeClapSnap,
            146 => Self::MazeButton,
            147 => Self::MazeNavInstructions,
            161 => Self::MazeBatteryLevel,
            162 => Self::MazeRotation,
            163 => Self::MazeSpeeds,
            164 => Self::MazeDistance,
            179 => Self::MazeEndOfMaze,
            177 => Self::MazeColours,
            178 => Self::MazeIncidence,
            208 => Self::SosClapSnap,
            228 => Self::SosSpeed,
            _ => Self::Undefined,
        }
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
            ControlByte::SosClapSnap => 208,
            ControlByte::SosSpeed => 228,
            ControlByte::Undefined => 255,
        }
    }
}
