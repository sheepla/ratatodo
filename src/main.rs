use action::Action;
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
    let events = EventHandler::new(10);
    let mut tui = Tui::new(terminal, events);
    let (action_sender, mut action_receiver) = tokio::sync::mpsc::unbounded_channel::<Action>();
    let dispatcher = Dispatcher::new(action_sender);

    tui.init()?;

    while !app.state.should_quit() {
        tokio::select! {
            Some(action) = action_receiver.recv() => {
                dispatcher.dispatch(&mut app.state, action).await;
            }

            Some(event) = tui.events.next() => {
                match event {
                    TerminalEvent::Tick => {
                        app.tick();
                        tui.draw(&mut app)?;
                    }
                    TerminalEvent::Key(key_event) => {
                        if let Some(action) = handle_key_events(key_event, &mut app.state) {
                            dispatcher.dispatch(&mut app.state, action).await;
                        }
                    }
                    TerminalEvent::Mouse(mouse_event) => {
                        if let Some(action) = handle_mouse_events(&mouse_event, &mut app.state) {
                            dispatcher.dispatch(&mut app.state, action).await;
                        }
                    }
                    TerminalEvent::Resize(_, _) => {}
                }
            }
        }
    }

    tui.exit()?;

    Ok(())
}
