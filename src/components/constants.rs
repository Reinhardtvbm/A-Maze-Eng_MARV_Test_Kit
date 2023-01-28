use std::f32::consts::PI;

pub struct Constants {}

// =================================================================================
// Maze painting constants

pub const MAZE_LINE_LENGTH: f32 = 80.0;
pub const MAZE_LINE_WIDTH: f32 = 5.0;
pub const MAZE_COL_WIDTH: f32 = MAZE_LINE_LENGTH + MAZE_LINE_WIDTH;
pub const MAZE_ROW_HEIGHT: f32 = MAZE_COL_WIDTH;
pub const MAZE_LEFT_JUSTIFICATION: f32 = MAZE_COL_WIDTH / 2.5;
pub const MAZE_TOP_JUSTIFICATION: f32 = 80.0 + (MAZE_ROW_HEIGHT / 2.5);

// =================================================================================

// =================================================================================
// Robot dimensions

pub const B_ISD: u16 = 65; // big inter-sensor distance (in mm)
pub const S_ISD: u16 = 15; // small inter-sensor distance (in mm)
pub const AXLE_DIST: i16 = 100; // length of the axle (in mm)

// =================================================================================

// =================================================================================
// Packets for comms

pub const CAL_COLOURS: [u8; 4] = [113, 0, 0, 0];
pub const CAL_CALIBRATED: [u8; 4] = [112, 0, 0, 0];

// =================================================================================

// =================================================================================
// QTP starting position defualts

pub const NINETY_DEGREES: f32 = PI / 2.0;
pub const DEFUALT_STARTING_POSITION: (f32, f32) = (0.1, 0.05);
pub const DEFUALT_COM_PORT: &str = "0";

// =================================================================================

// =================================================================================
// GUI constants

pub const SMALL_PADDING: f32 = 2.0;
pub const MEDIUM_PADDING: f32 = 4.0;
pub const LARGE_PADDING: f32 = 8.0;
pub const HUGE_PADDING: f32 = 12.0;

// =================================================================================

impl Constants {
    pub fn inter_sensor_distance() -> u16 {
        B_ISD
    }
}
