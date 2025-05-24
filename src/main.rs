use dispatcher::Dispatcher;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::App,
    event::{EventHandler, TerminalEvent},
    handler::{handle_key_events, handle_mouse_events},
    tui::Tui,
};

pub mod action;
pub mod app;
pub mod dispatcher;
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
    color_eyre::install()?;

    let mut app = App::init()?;

    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let dispatcher = Dispatcher::new();
    let mut tui = Tui::new(terminal, events);

    tui.init()?;

    while !app.state.should_quit() {
        tui.draw(&mut app)?;

        if let Some(event) = tui.events.next().await {
            match event {
                TerminalEvent::Tick => app.tick(),
                TerminalEvent::Key(key_event) => {
                    if let Some(action) = handle_key_events(key_event, &mut app.state) {
                        dispatcher.dispatch(action, &mut app.state).await;
                    }
                }
                TerminalEvent::Mouse(mouse_event) => {
                    if let Some(action) = handle_mouse_events(&mouse_event, &mut app.state) {
                        dispatcher.dispatch(action, &mut app.state).await;
                    }
                }
                TerminalEvent::Resize(_, _) => {}
            }
        }
    }

    tui.exit()?;

    Ok(())
}
