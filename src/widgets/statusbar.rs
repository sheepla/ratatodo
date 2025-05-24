use ratatui::prelude::*;
use ratatui::widgets::{Block, Padding, Paragraph, Widget};

use crate::state::SomeHeavyTaskState;

#[derive(Debug)]
pub struct StatusBar<'a> {
    paragraph: Paragraph<'a>,
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.paragraph.render(area, buf);
    }
}

impl<'a> StatusBar<'a> {
    pub fn new(state: &crate::state::State) -> Self {
        let paragraph = match state.some_heavy_task_state {
            SomeHeavyTaskState::Ready => Paragraph::new("Ready")
                .alignment(Alignment::Right)
                .fg(Color::Green)
                .bg(Color::Black),
            SomeHeavyTaskState::Loading => Paragraph::new("Loading...")
                .alignment(Alignment::Right)
                .fg(Color::Yellow)
                .bg(Color::Black),
        }
        .block(Block::new().padding(Padding::horizontal(2)));

        Self { paragraph }
    }
}
