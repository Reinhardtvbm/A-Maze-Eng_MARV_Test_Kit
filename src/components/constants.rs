use std::f32::consts::PI;

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

type Bytes = [u8; 4];

pub const HUB_START: Bytes = [0, 0, 0, 0];
pub const HUB_END_OF_MAZE: Bytes = [1, 0, 0, 0];

pub const IDLE_BUTTON_TOUCHED: Bytes = [16, 1, 30, 0];
pub const IDLE_BUTTON_NOT_TOUCHED: Bytes = [16, 0, 0, 0];

pub const CAL_CALIBRATED: Bytes = [112, 0, 0, 0];
pub const CAL_OPERATIONAL_VELOCITY: Bytes = [96, 50, 0, 0];
pub const CAL_BATTERY_LEVEL: Bytes = [97, 0, 0, 0];
pub const CAL_COLOURS: Bytes = [113, 0, 0, 0];
pub const CAL_BUTTON_TOUCHED: Bytes = [80, 1, 0, 0];
pub const CAL_BUTTON_NOT_TOUCHED: Bytes = [80, 0, 0, 0];

pub const MAZE_CLAPSNAP: Bytes = [145, 1, 0, 0];
pub const MAZE_CLAPSNAP_NONE: Bytes = [145, 0, 0, 0];
pub const MAZE_BUTTON_TOUCHED: Bytes = [146, 1, 0, 0];
pub const MAZE_BUTTON_NOT_TOUCHED: Bytes = [146, 0, 0, 0];
pub const MAZE_NAVCON_FORWARD: Bytes = [147, 30, 30, 0];
pub const MAZE_NAVCON_REVERSE: Bytes = [147, 30, 30, 1];
pub const MAZE_NAVCON_STOP: Bytes = [147, 0, 0, 0];
pub const MAZE_BATTERY_LEVEL: Bytes = [161, 0, 0, 0];
pub const MAZE_END_OF_MAZE: Bytes = [179, 0, 0, 0];

pub const SOS_SPEED: Bytes = [228, 0, 0, 0];
pub const SOS_CLAPSNAP: Bytes = [208, 1, 0, 0];
pub const SOS_CLAPSNAP_NONE: Bytes = [208, 0, 0, 0];

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
