use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub enum Window {
    Main,
    Navcon(QtpNo),
}

#[derive(Debug, Clone, Copy)]
pub enum QtpNo {
    Qtp1,
    Qtp2,
    Qtp3,
    Qtp4,
    Qtp5,
}

#[derive(Default)]
pub struct WindowHistory(VecDeque<Window>);

impl WindowHistory {
    pub fn new() -> Self {
        Self::default()
    }

    /// get the current window
    pub fn curr_window(&self) -> Option<Window> {
        self.0.front().copied()
    }

    /// push the next window
    pub fn push(&mut self, window: Window) {
        self.0.push_front(window);
    }

    /// goes back one window / pop and returns the _new_ current
    /// window
    pub fn pop(&mut self) -> Option<Window> {
        self.0.pop_front();
        self.curr_window()
    }
}
