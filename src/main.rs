pub mod app;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

use std::sync::{Arc, Mutex, RwLock};

use anyhow::Result;
use app::{App, RunningState};
use event::{EventHandler, Message};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    let mut app = App::new();

    let app_arc = Arc::new(RwLock::new(app));

    // init terminal user interface
    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;

    let app_clone = Arc::clone(&app_arc);
    let events = EventHandler::new(250, app_clone); // TODO: experiment on tick_rate

    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while app_arc.read().unwrap().running_state == RunningState::Running {
        tui.draw(&app_arc.read().unwrap())?;

        let mut msg = tui.events.next();
        while msg.is_some() {
            msg = update(&mut app_arc.write().unwrap(), msg.unwrap());
        }
    }

    tui.exit()?;
    Ok(())
}
