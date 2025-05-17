use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, List, Widget},
};

use crate::{models::models::TodoEntryState, state::State};

#[derive(Debug)]
pub struct TodoEntryList<'a> {
    list_widet: ratatui::widgets::List<'a>,
}

impl<'a> Widget for TodoEntryList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Widget::render(self.list_widet, area, buf);
    }
}

impl<'a> TodoEntryList<'a> {
    pub fn new(state: &'a State) -> Self {
        let mut entry_rows = Vec::<Text>::new();

        // Render entry rows from entries data
        for (index, entry) in state.data.entries.iter().enumerate() {
            let mut text = Text::default();

            let indicator_span = Span::styled(
                entry.state.to_indicator(),
                Style::new().fg(Color::Green).bold(),
            );

            let title_span = match entry.state {
                TodoEntryState::InComplete => {
                    if state.cursor == index {
                        Span::styled(entry.title.as_str(), Style::new().bold().fg(Color::Black))
                            .bg(Color::White)
                    } else {
                        Span::styled(entry.title.as_str(), Style::new().bold().fg(Color::White))
                    }
                }
                TodoEntryState::Completed => {
                    if state.cursor == index {
                        Span::styled(
                            entry.title.as_str(),
                            Style::new()
                                .add_modifier(Modifier::CROSSED_OUT)
                                .fg(Color::Black)
                                .bg(Color::White),
                        )
                    } else {
                        Span::styled(
                            entry.title.as_str(),
                            Style::new()
                                .fg(Color::Gray)
                                .add_modifier(Modifier::CROSSED_OUT),
                        )
                    }
                }
            };

            text.push_span(Span::raw(" "));
            text.push_span(indicator_span);
            text.push_span(Span::raw(" "));
            text.push_span(title_span);

            entry_rows.push(text);
        }

        Self {
            list_widet: List::default()
                .items(entry_rows)
                .highlight_style(Style::new().reversed().bold())
                .highlight_symbol("> ")
                .block(
                    Block::new()
                        // Border style
                        .borders(Borders::all())
                        .border_style(Style::default().fg(Color::DarkGray))
                        // Title style
                        .title("Todo List")
                        .title_style(Style::default().fg(Color::Gray))
                        .title_alignment(Alignment::Center),
                ),
        }
    }
}
