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

use crate::{components::colour::Colour, gui::maze::MazeLineMap};

pub fn paint_navcon_qtp_1(ui: &mut Ui) {
    // INITIALISE THE MAZE
    let mut maze_map = MazeLineMap::new(4, 1);

    maze_map
        .add_column(vec![
            Colour::Black,
            Colour::Green,
            Colour::White,
            Colour::Red,
            Colour::Black,
        ])
        .unwrap();

    for _ in 0..4 {
        maze_map.add_row(vec![Colour::Black; 2]).unwrap();
    }

    // PAINT THE MAZE ON UI
    maze_map.paint(ui);

    ui.painter().circle_filled(
        Pos2::new(100.0, 100.0),
        2.5,
        Color32::from_rgb(100, 100, 100),
    );
}
