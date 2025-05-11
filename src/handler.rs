use crate::{
    action::Action,
    models::models::TodoEntryState,
    state::{State, WidgetFocus},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

pub fn handle_key_events(key_event: KeyEvent, state: &State) -> Option<Action> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Char('q') => Some(Action::Quit),
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                Some(Action::Quit)
            } else {
                None
            }
        }
        // Move cursor up or down when focusing on the list view
        KeyCode::Up | KeyCode::Char('k') => match state.widget_focus {
            WidgetFocus::ListView => Some(Action::MoveCursor(-1)),
            WidgetFocus::TextArea => None,
        },
        KeyCode::Down | KeyCode::Char('j') => match state.widget_focus {
            WidgetFocus::ListView => Some(Action::MoveCursor(1)),
            WidgetFocus::TextArea => None,
        },

        // Toggle ToDo state
        KeyCode::Char(' ') => match state.widget_focus {
            WidgetFocus::ListView => Some(Action::ToggleCurrentEntryState),
            WidgetFocus::TextArea => None,
        },

        // Enter to text area like Vim's INSERT mode
        KeyCode::Char('i') | KeyCode::Char('a') => match state.widget_focus {
            WidgetFocus::ListView => Some(Action::MoveWidgetFocus(WidgetFocus::TextArea)),
            WidgetFocus::TextArea => None,
        },

        // Delete currrent entry
        KeyCode::Char('x') => match state.widget_focus {
            WidgetFocus::ListView => Some(Action::DeleteCurrentEntry),
            WidgetFocus::TextArea => None,
        },

        // Leave text area like Vim's NORMAL mode
        KeyCode::Esc => match state.widget_focus {
            WidgetFocus::ListView => None,
            WidgetFocus::TextArea => Some(Action::MoveWidgetFocus(WidgetFocus::ListView)),
        },

        _ => None,
    }
}

pub fn handle_mouse_events(mouse_event: &MouseEvent, state: &State) -> Option<Action> {
    match mouse_event.kind {
        MouseEventKind::ScrollDown => match state.widget_focus {
            WidgetFocus::ListView => Some(Action::MoveCursor(1)),
            WidgetFocus::TextArea => None,
        },
        MouseEventKind::ScrollUp => match state.widget_focus {
            WidgetFocus::ListView => Some(Action::MoveCursor(1)),
            WidgetFocus::TextArea => None,
        },
        _ => None,
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
    }
}
