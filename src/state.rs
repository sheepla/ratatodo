use crate::models::models::{TodoData, TodoEntryState};

#[derive(Debug, Default)]
pub struct State {
    pub should_quit: bool,
    pub widget_focus: WidgetFocus,
    pub cursor: usize,
    pub data: TodoData,
}

#[derive(Debug, Default)]
pub enum WidgetFocus {
    #[default]
    ListView,
    TextArea,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn move_cursor(&mut self, delta: i32) {
        let new_cursor = self.cursor as i32 + delta;

        if new_cursor < 0 {
            self.cursor = 0;
        } else if new_cursor >= self.data.entries.len() as i32 {
            self.cursor = self.data.entries.len().saturating_sub(1);
        } else {
            self.cursor = new_cursor as usize;
        }
    }

    pub fn delete_current_entry(&mut self) {
        if self.cursor < self.data.entries.len() {
            self.data.entries.remove(self.cursor);

            if self.cursor >= self.data.entries.len() && self.cursor > 0 {
                self.cursor -= 1;
            }
        }
    }

    pub fn toggle_current_entry_state(&mut self) {
        let index = self.cursor;

        if let Some(entry) = self.data.entries.get_mut(index) {
            entry.state = match entry.state {
                TodoEntryState::Completed => TodoEntryState::InComplete,
                TodoEntryState::InComplete => TodoEntryState::Completed,
            }
        }
    }
}
