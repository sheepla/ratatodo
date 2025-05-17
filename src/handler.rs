use crate::{
    action::Action,
    state::{State, WidgetFocus},
};
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use tui_textarea::TextArea;

pub fn handle_key_events(key_event: KeyEvent, state: &State) -> Option<Action> {
    match state.widget_focus {
        WidgetFocus::ListView => match key_event.code {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                Some(Action::Quit)
            }
            KeyCode::Up | KeyCode::Char('k') => Some(Action::MoveCursor(-1)),
            KeyCode::Down | KeyCode::Char('j') => Some(Action::MoveCursor(1)),
            KeyCode::Char(' ') => Some(Action::ToggleCurrentEntryState),
            KeyCode::Char('i') | KeyCode::Char('a') => {
                Some(Action::MoveWidgetFocus(WidgetFocus::TextArea))
            }
            KeyCode::Char('x') => Some(Action::DeleteCurrentEntry),
            _ => None,
        },
        WidgetFocus::TextArea => match key_event.code {
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                Some(Action::Quit)
            }
            KeyCode::Esc => Some(Action::MoveWidgetFocus(WidgetFocus::ListView)),
            _ => Some(Action::InputInTextArea(key_event)),
        },
    }
}

pub fn handle_mouse_events(mouse_event: &MouseEvent, state: &State) -> Option<Action> {
    match state.widget_focus {
        WidgetFocus::ListView => match mouse_event.kind {
            MouseEventKind::ScrollUp => Some(Action::MoveCursor(-1)),
            MouseEventKind::ScrollDown => Some(Action::MoveCursor(1)),
            _ => None,
        },
        WidgetFocus::TextArea => None,
    }
}

pub fn handle_actions(action: Action, state: &mut State) {
    match action {
        Action::MoveWidgetFocus(widget_focus) => state.widget_focus = widget_focus,
        Action::MoveCursor(delta) => state.move_cursor(delta),
        Action::Quit => {
            state.quit();
        }
        Action::DeleteCurrentEntry => {
            state.delete_current_entry();
        }
        Action::ToggleCurrentEntryState => {
            state.toggle_current_entry_state();
        }
        Action::InputInTextArea(key_event) => {
            if key_event.code == KeyCode::Enter {
                // Accept line
                let text = state.textarea.lines().join("\n");

                if text.trim().is_empty() {
                    return; // Ignore blank input
                }

                state.add_entry(&text); // Add an entry
                state.textarea = TextArea::new(vec![]); // Clear textarea
            } else {
                // Input text
                state.textarea.input(key_event);
            }
        }
    }
}
