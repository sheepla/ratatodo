use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{
    state::State,
    widgets::{
        statusbar::StatusBar, todo_entry_listview::TodoEntryList,
        todo_entry_textarea::TodoEntryTextArea,
    },
};

pub fn render_screen(state: &State, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3), // TextArea
            Constraint::Fill(1),   // List
            Constraint::Length(1), // StatusBar
        ])
        .split(frame.area());

    let textarea = TodoEntryTextArea::new(state);
    let list = TodoEntryList::new(state);
    let status_bar = StatusBar::new(state);

    frame.render_widget(textarea, layout[0]);
    frame.render_widget(list, layout[1]);
    frame.render_widget(status_bar, layout[2]);
}
