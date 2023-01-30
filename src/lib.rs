pub mod components {
    pub mod adjacent_bytes;
    pub mod buffer;
    pub mod colour;
    pub mod comm_port;
    pub mod constants;
    pub mod packet;
    pub mod state;
}

pub mod subsystems {
    pub mod state_navigation {
        pub mod navcon;
        pub mod snc;
    }

    pub mod motor_subsystem {
        pub mod mdps;
        pub mod wheel;
    }

    pub mod sensor_subsystem {
        pub mod ss;
    }

    pub mod channel;
    pub mod comms_channel;
    pub mod sensor_positions;
    pub mod serial_relay;
    pub mod system;
}

pub mod gui {
    pub mod entry_window;
    pub mod gui;
    pub mod maze;
    pub mod window_stack;

    pub mod test_windows {
        pub mod navcon {
            pub mod qtp1;
        }

        pub mod snc {
            pub mod qtp1;
        }
    }
}
