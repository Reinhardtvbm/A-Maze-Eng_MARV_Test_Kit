//! # NAVCON QTP 1
//!
//! Tests whether the NAVCON responds correctly to
//! a red OR green line, when they are encountered
//! at an angle of incidence less than or equal to
//! five degrees (<= 5)

extern crate eframe;

use eframe::{
    egui::Ui,
    epaint::{Color32, Pos2},
};

use crate::{
    components::{
        colour::Colour,
        constants::{MAZE_LEFT_JUSTIFICATION, MAZE_TOP_JUSTIFICATION},
    },
    gui::maze::MazeLineMap,
};

pub fn paint_navcon_qtp_1(ui: &mut Ui, sensor_pos: [(f32, f32); 5]) -> MazeLineMap {
    // INITIALISE THE MAZE
    let mut maze_map = MazeLineMap::new(4, 1);

    maze_map
        .add_column(vec![
            Colour::Black,
            Colour::Green,
            Colour::Blue,
            Colour::Red,
            Colour::Black,
        ])
        .unwrap();

    for _ in 0..4 {
        maze_map.add_row(vec![Colour::Black; 2]).unwrap();
    }

    // PAINT THE MAZE ON UI
    maze_map.paint(ui);

    sensor_pos.into_iter().for_each(|(x, y)| {
        ui.painter().circle_filled(
            Pos2::new(x + MAZE_LEFT_JUSTIFICATION, y + MAZE_TOP_JUSTIFICATION),
            2.5,
            Color32::from_rgb(100, 100, 100),
        );
    });

    maze_map
}
