mod api;
pub mod app;
pub mod event;
pub mod logging;
pub mod tui;
pub mod ui;
pub mod update;

use std::panic;
use std::sync::{Arc, RwLock};

use crate::update::update;
use anyhow::Result;
use app::{App, RunningState};
use event::EventHandler;
use human_panic::{handle_dump, print_msg, Metadata};
use ratatui::{backend::CrosstermBackend, Terminal};
use tracing::error;
use tui::Tui;

use self::api::BlockyApi;
use self::logging::initialize_logging;

fn main() -> Result<()> {
    initialize_logging()?;
    initialize_panic_handler()?;

    let app = App::new();

    let app_arc = Arc::new(RwLock::new(app));

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

fn initialize_panic_handler() -> Result<()> {
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        if let Err(r) = crate::tui::Tui::reset() {
            error!("Unable to reset terminal to default state {:?}", r);
        }
        let msg = format!(
            "{:?} at {:?}",
            panic_info.payload().downcast_ref::<&str>(),
            panic_info.location()
        );
        eprintln!("{}", msg);
        let meta = Metadata {
            version: env!("CARGO_PKG_VERSION").into(),
            name: env!("CARGO_PKG_NAME").into(),
            authors: env!("CARGO_PKG_AUTHORS").replace(':', ", ").into(),
            homepage: env!("CARGO_PKG_HOMEPAGE").into(),
        };
        let file_path = handle_dump(&meta, panic_info);
        print_msg(file_path, &meta).expect("human-panic: printing error message to console failed");
        tracing::error!("Error: {}", msg);
        panic_hook(panic_info)
    }));
    Ok(())
}
