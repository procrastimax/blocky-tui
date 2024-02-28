pub mod action;
pub mod api;
pub mod app;
pub mod logging;
pub mod port_check;
pub mod tui;
pub mod ui;
pub mod update;

use std::panic;

use anyhow::Result;
use human_panic::{handle_dump, print_msg, Metadata};
use tracing::{debug, error, info};

use self::app::App;
use self::logging::initialize_logging;

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logging()?;
    info!("----------- STARTING BLOCKY TUI -----------");

    initialize_panic_handler()?;

    let mut app = App::new()?;
    info!("initialization done");
    let result = app.run().await;
    if let Err(r) = result {
        error!("Main App Error: {}", r.to_string());
        return Err(r);
    }
    Ok(())
}

fn initialize_panic_handler() -> Result<()> {
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        match crate::tui::Tui::new() {
            Ok(mut t) => {
                if let Err(r) = t.exit() {
                    error!("Unable to reset terminal to default state {:?}", r);
                }
            }
            Err(r) => error!("Unable to exit terminal: {r:?}"),
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
        error!("Paniced Error: {}", msg);
        panic_hook(panic_info)
    }));
    debug!("initialized panic handler hook");
    Ok(())
}
