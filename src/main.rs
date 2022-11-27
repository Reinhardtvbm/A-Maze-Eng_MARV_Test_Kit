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
    mod snc {
        pub mod navcon;
        pub mod snc;
    }

    mod mdps {
        pub mod mdps;
        pub mod wheel;
    }

    mod ss {
        pub mod ss;
    }

    pub mod system;
}

fn main() {
    println!("Hello MARV!");

    let mut system = System::new();

    system.run();

    println!("{:?}", system);
}
