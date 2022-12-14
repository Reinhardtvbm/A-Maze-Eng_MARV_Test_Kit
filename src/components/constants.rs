pub struct Constants {}

pub const MAZE_LINE_LENGTH: f32 = 80.0;
pub const MAZE_LINE_WIDTH: f32 = 5.0;
pub const MAZE_COL_WIDTH: f32 = MAZE_LINE_LENGTH + MAZE_LINE_WIDTH;
pub const MAZE_ROW_HEIGHT: f32 = MAZE_COL_WIDTH;
pub const MAZE_LEFT_JUSTIFICATION: f32 = MAZE_COL_WIDTH / 2.5;
pub const MAZE_TOP_JUSTIFICATION: f32 = 40.0 + (MAZE_ROW_HEIGHT / 2.5);
pub const B_ISD: u16 = 65; // in mm
pub const S_ISD: u16 = 15; // in mm
pub const AXLE_DIST: i16 = 100; // in mm

impl Constants {
    pub fn inter_sensor_distance() -> u16 {
        B_ISD
    }
}
