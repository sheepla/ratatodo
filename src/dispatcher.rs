use crossterm::event::KeyCode;

use crate::{
    action::Action,
    state::{SomeHeavyTaskState, State},
};

#[derive(Debug)]
pub struct Dispatcher {
    // TODO
}

impl Dispatcher {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn dispatch(&self, action: Action, state: &mut State) {
        match action {
            Action::MoveWidgetFocus(focus) => state.set_widget_focus(focus),
            Action::MoveCursor(delta) => state.move_cursor(delta),
            Action::Quit => {
                state.set_quit();
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
                    let text = state.get_textarea_content();

                    if text.trim().is_empty() {
                        return; // Ignore blank input
                    }

                    state.add_entry(&text);
                    state.clear_textarea();
                } else {
                    // Input text
                    state.input_to_textarea(key_event);
                }
            }
            Action::RunSomeHeavyTask => {
                state.some_heavy_task_state = SomeHeavyTaskState::Loading;

                // do_some_heavy_task().await;

                state.some_heavy_task_state = SomeHeavyTaskState::Ready;
            }
        }
    }
}

// async fn do_some_heavy_task() {
//     tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
// }
