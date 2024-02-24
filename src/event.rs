use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{
    sync::{
        mpsc::{self},
        Arc, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use crate::app::App;

#[derive(Debug, PartialEq)]
pub enum Message {
    CycleFocusUp,   // Move focus to next tile in UI
    CycleFocusDown, // Move focus to previous tile in UI
    JumpToTile(u8),
    EnableDNSBlocking,  // enables DNS blocking
    DisableDNSBlocking, // disable DNS blocking
    SubmitDNSQuery,     // sends DNS query to blocky
    RefreshLists,       // Refresh blocking lists
    UpdateTile,         // Update current Tile (or all app information)
    Key(KeyEvent),
    Quit, // quits application
}

/// Terminal Event/ Message Handler
#[derive(Debug)]
pub struct EventHandler {
    /// Message sender channel.
    sender: mpsc::Sender<Message>,
    /// Message receiver channel.
    receiver: mpsc::Receiver<Message>,
    /// Event handler thread.
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Construct a new instance of ['EventHandler']
    pub fn new(tick_rate: u64, app: Arc<RwLock<App>>) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            Event::Key(e) => {
                                let msg = Self::handle_key(&app, e);
                                if let Some(msg) = msg {
                                    sender.send(msg)
                                } else {
                                    Ok(())
                                }
                            }
                            Event::Mouse(_e) => Ok(()),
                            Event::Resize(_w, _h) => Ok(()),
                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }
                    if last_tick.elapsed() >= tick_rate {
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    fn handle_key(app_arc: &Arc<RwLock<App>>, key: KeyEvent) -> Option<Message> {
        let app = app_arc.read().unwrap();
        match key.code {
            KeyCode::Esc => Some(Message::Quit),
            KeyCode::Char('q') => {
                if !app.is_currently_editing {
                    Some(Message::Quit)
                } else {
                    Some(Message::Key(key))
                }
            }
            KeyCode::Char('c') => {
                if key.modifiers == KeyModifiers::CONTROL {
                    Some(Message::Quit)
                } else {
                    Some(Message::Key(key))
                }
            }
            KeyCode::Char(val) => {
                if !app.is_currently_editing && val.is_numeric() {
                    // subtract 48 as u8, since the char->u8 conversion converts to ascii code
                    // so char('1') is 49 in u8
                    Some(Message::JumpToTile((val as u8) - 48u8))
                } else {
                    Some(Message::Key(key))
                }
            }
            KeyCode::Tab => Some(Message::CycleFocusUp),
            KeyCode::BackTab => Some(Message::CycleFocusDown),
            _ => None,
        }
    }

    pub fn next(&self) -> Option<Message> {
        match self.receiver.recv() {
            Ok(msg) => Some(msg),
            Err(_) => None,
        }
    }
}
