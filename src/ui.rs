use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{
    state::State, widgets::textarea::TodoEntryTextArea, widgets::todo_entry_listview::TodoEntryList,
};

pub fn render_screen(state: &mut State, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
        .split(frame.area());

    let textarea = TodoEntryTextArea::new(state);
    let list = TodoEntryList::new(state);

    frame.render_widget(textarea, layout[0]);
    frame.render_widget(list, layout[1]);
}
