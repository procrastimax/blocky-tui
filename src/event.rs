use crossterm::event::{self, Event, KeyEvent, MouseEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Message {
    // TODO: implement entering DNS query input
    CycleFocusUp,       // Move focus to next tile in UI
    CycleFocusDown,     // Move focus to previous tile in UI
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
    pub fn new(tick_rate: u64) -> Self {
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
                                // TODO: parse keys to known messages, else send keypress message
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(Message::Key(e))
                                } else {
                                    Ok(()) // ignore KeyEventKind::Release on windows
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

    pub fn next(&self) -> Result<Message> {
        Ok(self.receiver.recv()?)
    }
}
