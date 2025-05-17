use color_eyre::owo_colors::OwoColorize;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Widget};
use tui_textarea::TextArea;

use crate::state::{State, WidgetFocus};

pub struct TodoEntryTextArea<'a> {
    textarea: TextArea<'a>,
}

impl<'a> Widget for TodoEntryTextArea<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.textarea.render(area, buf);
    }
}

impl<'a> TodoEntryTextArea<'a> {
    pub fn new(state: &mut crate::state::State) -> Self {
        // Setup styles
        state.textarea.set_cursor_style(Style::default());
        state.textarea.set_placeholder_text("Todo Title");
        let border_color = match state.widget_focus {
            WidgetFocus::TextArea => Color::Cyan,
            WidgetFocus::ListView => Color::DarkGray,
        };
        state.textarea.set_block(
            Block::new()
                .borders(Borders::all())
                .border_style(Style::default().fg(border_color)),
        );

        Self {
            textarea: state.textarea.to_owned(),
        }
    }
}
