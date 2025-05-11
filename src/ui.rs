use ratatui::Frame;

use crate::{
    state::State,
    widgets::todo_entry_listview::TodoEntryList,
};

pub fn render_screen(state: &mut State, frame: &mut Frame) {
    let list = TodoEntryList::new(state);
    frame.render_widget(list, frame.area())
}
