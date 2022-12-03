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
    pub mod gui;
}

fn main() {
    println!("Hello MARV!");

    let mut system = System::new();

    system.run();
}
