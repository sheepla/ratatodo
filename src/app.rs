use crate::{
    services::cache::{load_todo_data, save_todo_data},
    state::{self, State},
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Cache Error: {0}")]
    Cache(#[from] crate::services::cache::CacheError),
}

#[derive(Debug)]
pub struct App {
    pub state: State,
}

impl App {
    pub fn init() -> Result<Self, AppError> {
        let mut state = state::State::new();
        state.data = load_todo_data()?;

        Ok(Self { state })
    }

    pub fn tick(&self) {}
}

impl Drop for App {
    fn drop(&mut self) {
        save_todo_data(&self.state.data).expect("failed to save todo entries data");
    }
}
