//! # Maze struct
//!
//! Contains a representation of a maze to be traversed
//! during testing

extern crate eframe;

use eframe::{
    egui::Ui,
    epaint::{Color32, Pos2, Rect, Rounding, Stroke, Vec2},
};

use crate::components::colour::Colour;
use crate::components::constants::*;

#[derive(Debug)]
pub enum MazeError {
    OutOfBounds,
    InvalidVecLength,
    ColumnsFull,
    RowsFull,
}

fn paint_rect(center_x: f32, center_y: f32, width: f32, height: f32, r: u8, g: u8, b: u8, ui: &Ui) {
    ui.painter().rect(
        Rect::from_center_size(Pos2::new(center_x, center_y), Vec2::new(width, height)),
        Rounding::default(),
        Color32::from_rgb(r, g, b),
        Stroke::default(),
    );
}

/// # MazeLineMap
/// ## A map of the horizontal and vertical lines that make up a maze
///
/// This struct maps the maze as follows:
///       maze      distance (f32)
///     -- -- --         5
///    |  |  |  |        80
///     -- -- --         5
///    |  |  |  |        80
///     -- -- --         5
///    |  |  |  |        80
///     -- -- --         5
/// Where the horizontal lines of the maze are represented by '--', and
/// the vertical lines in the maze are represented by '|' in the figure
/// above.
pub struct MazeLineMap {
    columns: Vec<Column>,
    rows: Vec<Row>,
    height: usize,
    width: usize,
}

impl MazeLineMap {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            columns: Vec::with_capacity(width),
            rows: Vec::with_capacity(height),
            height,
            width,
        }
    }

    pub fn add_column(&mut self, column: Vec<Colour>) -> Result<(), MazeError> {
        if column.len() != self.height + 1 {
            Err(MazeError::InvalidVecLength)
        } else if self.columns.len() == self.width {
            Err(MazeError::ColumnsFull)
        } else {
            self.columns.push(Column(
                column.into_iter().map(|colour| Line(colour)).collect(),
            ));

            Ok(())
        }
    }

    pub fn add_row(&mut self, row: Vec<Colour>) -> Result<(), MazeError> {
        if row.len() != self.width + 1 {
            Err(MazeError::InvalidVecLength)
        } else if self.rows.len() == self.height {
            Err(MazeError::RowsFull)
        } else {
            self.rows
                .push(Row(row.into_iter().map(|colour| Line(colour)).collect()));

            Ok(())
        }
    }

    pub fn get_colour_from_coord(&self, x: f32, y: f32) -> Option<Colour> {
        //println!("accessing coord: ({}, {})", x, y);
        // x: <-->
        //
        // mod by (MAZE_LINE_LENGTH + MAZE_LINE_WIDTH) to get block indices

        // if x > 85.0 {
        //     println!("beep");
        // }

        let col_index = x.floor() as usize / (MAZE_LINE_LENGTH + MAZE_LINE_WIDTH) as usize;
        let row_index = y.floor() as usize / (MAZE_LINE_LENGTH + MAZE_LINE_WIDTH) as usize;

        // each row will contain:
        //     -- -- --
        //    |  |  |  |
        //    5805805805
        // and each column will contain:
        //     --  5  (MAZE_LINE_WIDTH)
        //    |    80 (MAZE_LINE_LENGTH)
        //     --  5
        //    |    80
        //     --  5
        //    |    80
        //     --  5
        //
        // so then each [col_index][row_index] will have:
        //  ----- :  horizontal line (-)  ↑
        // |                             085 pixels
        // |      : vertical line (|)     ↓

        // if we are outside the maze then just return white
        if col_index > self.columns.len() || row_index > self.rows.len() {
            return Some(Colour::White);
        }

        // get coords within block
        let x_in_block = x - (MAZE_LINE_LENGTH + MAZE_LINE_WIDTH) * col_index as f32;
        let y_in_block = y - (MAZE_LINE_LENGTH + MAZE_LINE_WIDTH) * row_index as f32;

        // if x_in_block > 5 && y_in_block <= 5 then the point is within the horizontal line, but
        // if y_in_block > 5 && x_in_block <= 5 then the point is within the vertical line
        // otherwise it is within the block itself, and we can just return Colour::White.
        if x_in_block > MAZE_LINE_WIDTH && y_in_block <= MAZE_LINE_WIDTH {
            // point is in horizontal line (contained in columns)
            self.columns[col_index].get(row_index)
        } else if y_in_block > MAZE_LINE_WIDTH && x_in_block <= MAZE_LINE_WIDTH {
            // point is in vertical line (contained in rows)
            self.rows[row_index].get(col_index)
        } else {
            Some(Colour::White)
        }
    }

    pub fn paint(&self, ui: &Ui) {
        // PAINT BACKGROUND RECT:
        let backg_center_x = MAZE_LEFT_JUSTIFICATION + (self.width as f32 * MAZE_COL_WIDTH) / 2.0;
        let backg_center_y = MAZE_TOP_JUSTIFICATION + (self.height as f32 * MAZE_ROW_HEIGHT) / 2.0;
        let backg_width = (0.5 + self.width as f32) * MAZE_COL_WIDTH;
        let backg_height = (0.5 + self.height as f32) * MAZE_ROW_HEIGHT;

        paint_rect(
            backg_center_x,
            backg_center_y,
            backg_width,
            backg_height,
            255,
            255,
            255,
            ui,
        );

        // PAINT VERTICAL LINES
        self.rows.iter().enumerate().for_each(|(row_index, row)| {
            row.0.iter().enumerate().for_each(|(col_index, v_line)| {
                let rgb: (u8, u8, u8) = v_line.0.into();

                paint_rect(
                    MAZE_LEFT_JUSTIFICATION + (MAZE_COL_WIDTH * col_index as f32),
                    MAZE_TOP_JUSTIFICATION + (MAZE_ROW_HEIGHT * (0.5 + row_index as f32)),
                    MAZE_LINE_WIDTH,
                    MAZE_LINE_LENGTH,
                    rgb.0,
                    rgb.1,
                    rgb.2,
                    ui,
                );
            })
        });

        // PAINT HORIZONTAL LINES
        self.columns
            .iter()
            .enumerate()
            .for_each(|(col_index, col)| {
                col.0.iter().enumerate().for_each(|(row_index, h_line)| {
                    let rgb: (u8, u8, u8) = h_line.0.into();

                    paint_rect(
                        MAZE_LEFT_JUSTIFICATION + (MAZE_COL_WIDTH * (0.5 + col_index as f32)),
                        MAZE_TOP_JUSTIFICATION + (MAZE_ROW_HEIGHT * row_index as f32),
                        MAZE_LINE_LENGTH,
                        MAZE_LINE_WIDTH,
                        rgb.0,
                        rgb.1,
                        rgb.2,
                        ui,
                    );
                })
            });
    }
}

pub struct Line(Colour);

impl Line {
    pub fn new(colour: Colour) -> Self {
        Self(colour)
    }

    pub fn colour(&self) -> Colour {
        self.0
    }
}

pub struct Column(Vec<Line>);
pub struct Row(Vec<Line>);

impl Column {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn lines(&self) -> &Vec<Line> {
        &self.0
    }

    pub fn get(&self, index: usize) -> Option<Colour> {
        match self.0.get(index) {
            Some(line) => Some(line.0),
            None => None,
        }
    }
}

impl Row {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn lines(&self) -> &Vec<Line> {
        &self.0
    }

    pub fn get(&self, index: usize) -> Option<Colour> {
        match self.0.get(index) {
            Some(line) => Some(line.0),
            None => None,
        }
    }
}
