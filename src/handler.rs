
use crate::{
    action::Action,
    state::{State, WidgetFocus},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

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
            KeyCode::Char('r') => Some(Action::RunSomeHeavyTask),
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
