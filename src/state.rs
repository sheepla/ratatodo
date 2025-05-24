use crossterm::event::{KeyCode, KeyEvent};
use tui_textarea::TextArea;

use crate::{
    models::models::{TodoData, TodoEntry, TodoEntryState},
    widgets::statusbar,
};

#[derive(Debug, Default)]
pub struct State {
    quit: bool,
    pub widget_focus: WidgetFocus,
    pub cursor: usize,
    pub data: TodoData,
    textarea: tui_textarea::TextArea<'static>,
    pub some_heavy_task_state: SomeHeavyTaskState,
}

#[derive(Debug, Default, Clone)]
pub enum WidgetFocus {
    #[default]
    ListView,
    TextArea,
}

#[derive(Debug, Default)]
pub enum SomeHeavyTaskState {
    Loading,
    #[default]
    Ready,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }

    pub fn set_quit(&mut self) {
        self.quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.quit
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

    pub fn add_entry(&mut self, title: &str) {
        self.data.entries.push(TodoEntry {
            title: title.to_string(),
            state: TodoEntryState::InComplete,
        })
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

    pub fn set_widget_focus(&mut self, focus: WidgetFocus) {
        self.widget_focus = focus;
    }

    pub fn get_widget_focus(&self) -> WidgetFocus {
        self.widget_focus.clone()
    }

    pub fn get_textarea_content(&self) -> String {
        self.textarea.lines().join("\n")
    }

    pub fn clear_textarea(&mut self) {
        self.textarea = TextArea::new(vec![]);
    }

    pub fn input_to_textarea(&mut self, key_event: KeyEvent) {
        self.textarea.input(key_event);
    }

    pub fn get_textarea(&self) -> TextArea {
        self.textarea.to_owned()
    }
}
