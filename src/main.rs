mod api;
pub mod app;
pub mod event;
pub mod logging;
pub mod tui;
pub mod ui;
pub mod update;

use std::sync::{Arc, RwLock};

use crate::update::update;
use anyhow::Result;
use app::{App, RunningState};
use event::EventHandler;
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;

use self::api::BlockyApi;
use self::logging::initialize_logging;

fn main() -> Result<()> {
    initialize_logging()?;

    let app = App::new();

    let app_arc = Arc::new(RwLock::new(app));

    // TODO: add more loggin
    let backend = CrosstermBackend::new(std::io::stdout());

    let terminal = Terminal::new(backend)?;

    let api = BlockyApi::new("localhost", 53);

    let app_clone = Arc::clone(&app_arc);
    let events = EventHandler::new(250, app_clone); // HACK: experiment on tick_rate

    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while app_arc.read().unwrap().running_state == RunningState::Running {
        tui.draw(&app_arc.read().unwrap())?;

        let mut msg = tui.events.next();
        while msg.is_some() {
            msg = update(&mut app_arc.write().unwrap(), &api, msg.unwrap());
        }
    }

    tui.exit()?;
    Ok(())
}
