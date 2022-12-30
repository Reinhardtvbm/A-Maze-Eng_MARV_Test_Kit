use std::{fmt, ops::Not};

#[derive(Debug, Clone, Copy)]
pub enum SubsystemMode {
    Auto,
    Manual,
}

pub struct EntryWindow {
    snc_mode: SubsystemMode,
    ss_mode: SubsystemMode,
    mdps_mode: SubsystemMode,
}

impl EntryWindow {
    pub fn new() -> Self {
        Self {
            snc_mode: SubsystemMode::Auto,
            ss_mode: SubsystemMode::Auto,
            mdps_mode: SubsystemMode::Auto,
        }
    }

    pub fn toggle_snc_mode(&mut self) {
        self.snc_mode = !self.snc_mode;
    }

    pub fn toggle_ss_mode(&mut self) {
        self.ss_mode = !self.ss_mode;
    }

    pub fn toggle_mdps_mode(&mut self) {
        self.mdps_mode = !self.mdps_mode;
    }

    pub fn snc_mode(&self) -> SubsystemMode {
        self.snc_mode
    }

    pub fn ss_mode(&self) -> SubsystemMode {
        self.ss_mode
    }

    pub fn mdps_mode(&self) -> SubsystemMode {
        self.mdps_mode
    }
}

impl fmt::Display for SubsystemMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubsystemMode::Auto => write!(f, "automatic"),
            SubsystemMode::Manual => write!(f, "manual"),
        }
    }
}

impl Not for SubsystemMode {
    type Output = SubsystemMode;

    fn not(self) -> Self::Output {
        match self {
            SubsystemMode::Auto => SubsystemMode::Manual,
            SubsystemMode::Manual => SubsystemMode::Auto,
        }
    }
}
