use std::time::Duration;

use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

#[derive(Clone, Copy, Debug)]
pub enum TerminalEvent {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::UnboundedSender<TerminalEvent>,
    receiver: mpsc::UnboundedReceiver<TerminalEvent>,
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let _sender = sender.clone();

        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                  _ = _sender.closed() => {
                    break;
                  }
                  _ = tick_delay => {
                    _sender.send(TerminalEvent::Tick).unwrap();
                  }
                  Some(Ok(evt)) = crossterm_event => {
                    if let Some(event) = handle_crossterm_events(evt).await {
                      _sender.send(event).unwrap();
                    }
                  }
                };
            }
        });
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub async fn next(&mut self) -> Option<TerminalEvent> {
        self.receiver.recv().await
    }
}

async fn handle_crossterm_events(event: CrosstermEvent) -> Option<TerminalEvent> {
    match event {
        CrosstermEvent::Key(key) => {
            if key.kind == crossterm::event::KeyEventKind::Press {
                Some(TerminalEvent::Key(key))
            } else {
                None
            }
        }
        CrosstermEvent::Mouse(mouse) => Some(TerminalEvent::Mouse(mouse)),
        CrosstermEvent::Resize(x, y) => Some(TerminalEvent::Resize(x, y)),
        CrosstermEvent::FocusLost => None,
        CrosstermEvent::FocusGained => None,
        CrosstermEvent::Paste(_) => None,
    }
}
