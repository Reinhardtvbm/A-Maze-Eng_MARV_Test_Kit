mod components {
    pub mod adjacent_bytes;
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
    }

    mod ss {
        pub mod ss;
    }
}

mod maze {
    pub mod graph;
    pub mod maze;
}

fn main() {
    println!("Hello MARV!");
}
