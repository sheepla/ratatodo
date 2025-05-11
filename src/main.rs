use eyre::eyre;
use handler::{handle_actions, handle_mouse_events};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::App,
    event::{EventHandler, TerminalEvent},
    handler::handle_key_events,
    tui::Tui,
};

pub mod action;
pub mod app;
pub mod event;
pub mod handler;
pub mod models;
pub mod services;
pub mod state;
pub mod tui;
pub mod ui;
pub mod widgets;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut app = App::init()?;

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    tui.init()?;

    while !app.state.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next().await? {
            TerminalEvent::Tick => app.tick(),
            TerminalEvent::Key(key_event) => {
                if let Some(action) = handle_key_events(key_event, &mut app.state) {
                    handle_actions(action, &mut app.state);
                }
            }
            TerminalEvent::Mouse(mouse_event) => {
                if let Some(action) = handle_mouse_events(&mouse_event, &mut app.state) {
                    handle_actions(action, &mut app.state);
                }
            }
            TerminalEvent::Resize(_, _) => {}
        }
    }

    tui.exit()?;

    Ok(())
}
