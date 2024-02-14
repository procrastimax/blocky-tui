pub mod app;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

use anyhow::Result;
use app::{App, RunningState};
use event::{EventHandler, Message};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    let mut app = App::new();

    // init terminal user interface
    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250); // TODO: experiment on tick_rate

    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while app.running_state == RunningState::Running {
        tui.draw(&mut app)?;

        let msg = tui.events.next()?;
        update(&mut app, msg)
    }

    tui.exit()?;
    Ok(())
}
