use crate::subsystems::system::System;

mod components {
    pub mod adjacent_bytes;
    pub mod buffer;
    pub mod colour;
    pub mod comm_port;
    pub mod constants;
    pub mod packet;
    pub mod state;
}

mod subsystems {
    mod state_navigation {
        pub mod navcon;
        pub mod snc;
    }

    mod motor_subsystem {
        pub mod mdps;
        pub mod wheel;
    }

    mod sensor_subsystem {
        pub mod ss;
    }

    pub mod system;
}

mod gui {
    pub mod entry_window;
    pub mod gui;
    pub mod window_stack;
}

use gui::gui::MARVApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "A-Maze-Eng-MARV Test Kit",
        native_options,
        Box::new(|cc| Box::new(MARVApp::new(cc))),
    );
}
