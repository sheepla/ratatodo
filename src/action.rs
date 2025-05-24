use crossterm::event::KeyEvent;

use crate::state::WidgetFocus;

#[derive(Debug)]
pub enum Action {
    MoveWidgetFocus(WidgetFocus),
    MoveCursor(i32),
    DeleteCurrentEntry,
    ToggleCurrentEntryState,
    InputInTextArea(KeyEvent),
    RunSomeHeavyTask,
    Quit,
}
