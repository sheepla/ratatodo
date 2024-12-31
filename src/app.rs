use std::error;

use crate::state::State;

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub state: State,
}

impl Default for App {
    fn default(initial_state: &State) -> Self {
        Self {
            running: true,
            state: initial_state,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
